/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `MPMoviePlayerController` etc.

use super::yuv;
use crate::dyld::{ConstantExports, HostConstant};
use crate::frameworks::foundation::{ns_string, ns_url, NSInteger};
use crate::frameworks::uikit::ui_device::UIDeviceOrientation;
use crate::fs::GuestPath;
use crate::gles::gles11_raw as gles11;
use crate::gles::GLES;
use crate::objc::{
    id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr,
};
use crate::Environment;
use h264_decoder::Decoder;
use std::collections::VecDeque;
use std::io::Cursor;
use symphonia::core::codecs::{Decoder as _, CODEC_TYPE_H264};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

#[derive(Default)]
pub struct State {
    pub active_player: Option<id>,
    /// Various apps (e.g. Crash Bandicoot Nitro Kart 3D and Spore Origins)
    /// create or start a player and await some kind of notification, but can't
    /// handle it if that notification happens immediately. This queue lets us
    /// delay such notifications until the app next returns to the run loop,
    /// which seems to be late enough.
    pending_notifications: VecDeque<(&'static str, id)>,
}
impl State {
    fn get(env: &mut Environment) -> &mut Self {
        &mut env.framework_state.media_player.movie_player
    }
}

type MPMovieScalingMode = NSInteger;
type MPMoviePlaybackState = NSInteger;
type MPMovieLoadState = NSInteger;

// MPMoviePlaybackState values
const MPMoviePlaybackStateStopped: MPMoviePlaybackState = 0;
const MPMoviePlaybackStatePlaying: MPMoviePlaybackState = 1;
#[allow(dead_code)]
const MPMoviePlaybackStatePaused: MPMoviePlaybackState = 2;
#[allow(dead_code)]
const MPMoviePlaybackStateInterrupted: MPMoviePlaybackState = 3;
#[allow(dead_code)]
const MPMoviePlaybackStateSeekingForward: MPMoviePlaybackState = 4;
#[allow(dead_code)]
const MPMoviePlaybackStateSeekingBackward: MPMoviePlaybackState = 5;

// MPMovieLoadState values
const MPMovieLoadStateUnknown: MPMovieLoadState = 0;
const MPMovieLoadStatePlayable: MPMovieLoadState = 1 << 0;
const MPMovieLoadStatePlaythroughOK: MPMovieLoadState = 1 << 1;
#[allow(dead_code)]
const MPMovieLoadStateStalled: MPMovieLoadState = 1 << 2;

// Notification names
pub const MPMoviePlayerPlaybackDidFinishNotification: &str =
    "MPMoviePlayerPlaybackDidFinishNotification";
/// Apparently an undocumented, private API. Spore Origins uses it.
pub const MPMoviePlayerContentPreloadDidFinishNotification: &str =
    "MPMoviePlayerContentPreloadDidFinishNotification";
pub const MPMoviePlayerScalingModeDidChangeNotification: &str =
    "MPMoviePlayerScalingModeDidChangeNotification";
pub const MPMoviePlayerPlaybackStateDidChangeNotification: &str =
    "MPMoviePlayerPlaybackStateDidChangeNotification";
pub const MPMoviePlayerLoadStateDidChangeNotification: &str =
    "MPMoviePlayerLoadStateDidChangeNotification";

/// `NSNotificationName` values.
pub const CONSTANTS: ConstantExports = &[
    (
        "_MPMoviePlayerPlaybackDidFinishNotification",
        HostConstant::NSString(MPMoviePlayerPlaybackDidFinishNotification),
    ),
    (
        "_MPMoviePlayerContentPreloadDidFinishNotification",
        HostConstant::NSString(MPMoviePlayerContentPreloadDidFinishNotification),
    ),
    (
        "_MPMoviePlayerScalingModeDidChangeNotification",
        HostConstant::NSString(MPMoviePlayerScalingModeDidChangeNotification),
    ),
    (
        "_MPMoviePlayerPlaybackStateDidChangeNotification",
        HostConstant::NSString(MPMoviePlayerPlaybackStateDidChangeNotification),
    ),
    (
        "_MPMoviePlayerLoadStateDidChangeNotification",
        HostConstant::NSString(MPMoviePlayerLoadStateDidChangeNotification),
    ),
];

struct MPMoviePlayerControllerHostObject {
    // NSURL *
    content_url: id,
    playback_state: MPMoviePlaybackState,
    load_state: MPMovieLoadState,
    reader: Option<Box<dyn FormatReader>>,
    video_track_id: Option<u32>,
    decoder: Option<Decoder>,
    video_texture: Option<u32>,
    frame_dimensions: Option<(u32, u32)>,
}
impl HostObject for MPMoviePlayerControllerHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation MPMoviePlayerController: NSObject

// TODO: actual playback

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(MPMoviePlayerControllerHostObject {
        content_url: nil,
        playback_state: MPMoviePlaybackStateStopped,
        load_state: MPMovieLoadStateUnknown,
        reader: None,
        video_track_id: None,
        decoder: None,
        video_texture: None,
        frame_dimensions: None,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithContentURL:(id)url { // NSURL*
    log_dbg!(
        "[(MPMoviePlayerController*){:?} initWithContentURL:{:?} ({:?})]",
        this,
        url,
        ns_url::to_rust_path(env, url),
    );

    retain(env, url);
    let host_obj = env.objc.borrow_mut::<MPMoviePlayerControllerHostObject>(this);
    host_obj.content_url = url;

    if let Some(path) = ns_url::to_rust_path(env, url) {
        if let Ok(bytes) = env.fs.read(&GuestPath::from(path)) {
            let mss = MediaSourceStream::new(Box::new(Cursor::new(bytes)), Default::default());
            let format_opts = FormatOptions {
                enable_gapless: true,
                ..Default::default()
            };
            let metadata_opts = MetadataOptions::default();

            if let Ok(probed) = symphonia::default::get_probe().format(&Default::default(), mss, &format_opts, &metadata_opts) {
                host_obj.reader = Some(probed.format);
                if let Some(track) = host_obj.reader.as_ref().unwrap().tracks().iter().find(|t| t.codec_params.codec == CODEC_TYPE_H264) {
                    host_obj.video_track_id = Some(track.id);
                    if let Some(params) = &track.codec_params {
                        if let (Some(width), Some(height)) = (params.width, params.height) {
                            host_obj.frame_dimensions = Some((width as u32, height as u32));
                        }
                    }

                    if let Ok(decoder) = Decoder::new() {
                        host_obj.decoder = Some(decoder);
                        host_obj.load_state = MPMovieLoadStatePlayable | MPMovieLoadStatePlaythroughOK;

                        // Act as if loading immediately completed (Spore Origins waits for this).
                        State::get(env).pending_notifications.push_back(
                            (MPMoviePlayerContentPreloadDidFinishNotification, this)
                        );
                        State::get(env).pending_notifications.push_back(
                            (MPMoviePlayerLoadStateDidChangeNotification, this)
                        );
                    }
                }
            }
        }
    }

    this
}

- (())dealloc {
    let url = env.objc.borrow::<MPMoviePlayerControllerHostObject>(this).content_url;
    release(env, url);

    env.objc.dealloc_object(this, &mut env.mem);
}

- (id)contentURL {
    env.objc.borrow::<MPMoviePlayerControllerHostObject>(this).content_url
}

- (())setBackgroundColor:(id)_color { // UIColor*
    // TODO
}

- (())setScalingMode:(MPMovieScalingMode)_mode {
    // TODO
}

// Apparently an undocumented, private API, but Spore Origins uses it.
- (())setMovieControlMode:(NSInteger)_mode {
    // Game-specific hack :(
    // Spore Origins subscribes to the playback finished notification 0.2s after
    // starting playback, so it misses the notification we send. When it
    // subscribes, it also calls this method, so this is an opportunity to send
    // the notification again.
    if env.bundle.bundle_identifier().starts_with("com.ea.spore") {
        log!("Applying game-specific hack for Spore Origins: sending MPMoviePlayerPlaybackDidFinishNotification again.");
        State::get(env).pending_notifications.push_back(
            (MPMoviePlayerPlaybackDidFinishNotification, this)
        );
    }
    // As this is undocumented and we don't have real video playback yet, let's
    // ignore it otherwise.
}

// Another undocumented one! But some apps may still use it :/
// https://stackoverflow.com/a/1390079/2241008
- (())setOrientation:(UIDeviceOrientation)_orientation animated:(bool)_animated {

}

// MPMediaPlayback implementation
- (())play {
    log_dbg!("[(MPMoviePlayerController*){:?} play]", this);
    if let Some(old) = env.framework_state.media_player.movie_player.active_player {
        let _: () = msg![env; old stop];
    }
    assert!(env.framework_state.media_player.movie_player.active_player.is_none());
    // Movie player is retained by the runtime until it is stopped
    retain(env, this);
    env.framework_state.media_player.movie_player.active_player = Some(this);

    let host_obj = env.objc.borrow_mut::<MPMoviePlayerControllerHostObject>(this);
    if host_obj.video_texture.is_none() {
        if let Some((width, height)) = host_obj.frame_dimensions {
            let gles = env.window().get_internal_gl_ctx();
            let mut texture = 0;
            unsafe {
                gles.GenTextures(1, &mut texture);
                gles.BindTexture(gles11::TEXTURE_2D, texture);
                gles.TexImage2D(
                    gles11::TEXTURE_2D,
                    0,
                    gles11::RGB as _,
                    width as _,
                    height as _,
                    0,
                    gles11::RGB,
                    gles11::UNSIGNED_BYTE,
                    std::ptr::null(),
                );
                gles.TexParameteri(
                    gles11::TEXTURE_2D,
                    gles11::TEXTURE_MIN_FILTER,
                    gles11::LINEAR as _,
                );
                gles.TexParameteri(
                    gles11::TEXTURE_2D,
                    gles11::TEXTURE_MAG_FILTER,
                    gles11::LINEAR as _,
                );
            }
            host_obj.video_texture = Some(texture);
        }
    }

    // Update playback state
    host_obj.playback_state = MPMoviePlaybackStatePlaying;
    State::get(env).pending_notifications.push_back(
        (MPMoviePlayerPlaybackStateDidChangeNotification, this)
    );
}

- (())stop {
    log_dbg!("[(MPMoviePlayerController*){:?} stop]", this);
    assert!(this == env.framework_state.media_player.movie_player.active_player.take().unwrap());
    
    let host_obj = env.objc.borrow_mut::<MPMoviePlayerControllerHostObject>(this);
    if let Some(texture) = host_obj.video_texture.take() {
        let gles = env.window().get_internal_gl_ctx();
        unsafe {
            gles.DeleteTextures(1, &texture);
        }
    }

    // Update playback state
    host_obj.playback_state = MPMoviePlaybackStateStopped;
    State::get(env).pending_notifications.push_back(
        (MPMoviePlayerPlaybackStateDidChangeNotification, this)
    );
    
    release(env, this);
}

- (MPMoviePlaybackState)playbackState {
    env.objc.borrow::<MPMoviePlayerControllerHostObject>(this).playback_state
}

- (MPMovieLoadState)loadState {
    env.objc.borrow::<MPMoviePlayerControllerHostObject>(this).load_state
}

@end

};

/// For use by `NSRunLoop` via [super::handle_players]: check movie players'
/// status, send notifications if necessary.
pub(super) fn handle_players(env: &mut Environment) {
    if let Some(player) = State::get(env).active_player {
        let mut host_obj = env.objc.borrow_mut::<MPMoviePlayerControllerHostObject>(player);
        if host_obj.playback_state == MPMoviePlaybackStatePlaying {
            if let (Some(reader), Some(video_track_id), Some(decoder), Some(video_texture), Some((width, height))) = (
                host_obj.reader.as_mut(),
                host_obj.video_track_id,
                host_obj.decoder.as_mut(),
                host_obj.video_texture,
                host_obj.frame_dimensions,
            ) {
                match reader.next_packet() {
                    Ok(packet) => {
                        if packet.track_id() == video_track_id {
                            match decoder.decode(packet.data) {
                                Ok(Some(picture)) => {
                                    let mut rgb_buf = vec![0u8; (width * height * 3) as usize];
                                    yuv::yuv_to_rgb(
                                        picture.y_with_stride().0,
                                        picture.u_with_stride().0,
                                        picture.v_with_stride().0,
                                        width as usize,
                                        height as usize,
                                        &mut rgb_buf,
                                    );

                                    let gles = env.window().get_internal_gl_ctx();
                                    unsafe {
                                        gles.BindTexture(gles11::TEXTURE_2D, video_texture);
                                        gles.TexSubImage2D(
                                            gles11::TEXTURE_2D,
                                            0,
                                            0,
                                            0,
                                            width as _,
                                            height as _,
                                            gles11::RGB,
                                            gles11::UNSIGNED_BYTE,
                                            rgb_buf.as_ptr() as *const _,
                                        );
                                    }
                                }
                                Ok(None) => {}
                                Err(e) => {
                                    log!("H.264 decoding error: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(SymphoniaError::IoError(_)) => {
                        // End of stream
                        host_obj.playback_state = MPMoviePlaybackStateStopped;
                        State::get(env).pending_notifications.push_back((
                            MPMoviePlayerPlaybackDidFinishNotification,
                            player,
                        ));
                        let _: () = msg![env; player stop];
                    }
                    Err(e) => {
                        log!("Error reading packet: {:?}", e);
                    }
                }
            }
        }
    }

    while let Some(notif) = State::get(env).pending_notifications.pop_front() {
        let (name, object) = notif;
        let name = ns_string::get_static_str(env, name);
        let center: id = msg_class![env; NSNotificationCenter defaultCenter];
        // TODO: should there be some user info attached?
        let _: () = msg![env; center postNotificationName:name object:object];
    }
}
