/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `MPMoviePlayerController` etc.

use crate::dyld::{ConstantExports, HostConstant};
use crate::frameworks::foundation::{ns_string, ns_url, NSInteger};
use crate::frameworks::uikit::ui_device::UIDeviceOrientation;
use crate::objc::{
    id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr,
};
use crate::Environment;
use std::collections::VecDeque;

#[derive(Default)]
pub struct State {
    active_player: Option<id>,
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
const MPMoviePlaybackStatePaused: MPMoviePlaybackState = 2;
const MPMoviePlaybackStateInterrupted: MPMoviePlaybackState = 3;
const MPMoviePlaybackStateSeekingForward: MPMoviePlaybackState = 4;
const MPMoviePlaybackStateSeekingBackward: MPMoviePlaybackState = 5;

// MPMovieLoadState values
const MPMovieLoadStateUnknown: MPMovieLoadState = 0;
const MPMovieLoadStatePlayable: MPMovieLoadState = 1 << 0;
const MPMovieLoadStatePlaythroughOK: MPMovieLoadState = 1 << 1;
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
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithContentURL:(id)url { // NSURL*
    log!(
        "TODO: [(MPMoviePlayerController*){:?} initWithContentURL:{:?} ({:?})]",
        this,
        url,
        ns_url::to_rust_path(env, url),
    );

    retain(env, url);
    let host_obj = env.objc.borrow_mut::<MPMoviePlayerControllerHostObject>(this);
    host_obj.content_url = url;
    host_obj.load_state = MPMovieLoadStatePlayable | MPMovieLoadStatePlaythroughOK;

    // Act as if loading immediately completed (Spore Origins waits for this).
    State::get(env).pending_notifications.push_back(
        (MPMoviePlayerContentPreloadDidFinishNotification, this)
    );
    State::get(env).pending_notifications.push_back(
        (MPMoviePlayerLoadStateDidChangeNotification, this)
    );

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
    log!("TODO: [(MPMoviePlayerController*){:?} play]", this);
    if let Some(old) = env.framework_state.media_player.movie_player.active_player {
        let _: () = msg![env; old stop];
    }
    assert!(env.framework_state.media_player.movie_player.active_player.is_none());
    // Movie player is retained by the runtime until it is stopped
    retain(env, this);
    env.framework_state.media_player.movie_player.active_player = Some(this);

    // Update playback state
    env.objc.borrow_mut::<MPMoviePlayerControllerHostObject>(this).playback_state = MPMoviePlaybackStatePlaying;
    State::get(env).pending_notifications.push_back(
        (MPMoviePlayerPlaybackStateDidChangeNotification, this)
    );

    // Act as if playback immediately completed (various apps wait for this).
    env.objc.borrow_mut::<MPMoviePlayerControllerHostObject>(this).playback_state = MPMoviePlaybackStateStopped;
    State::get(env).pending_notifications.push_back(
        (MPMoviePlayerPlaybackDidFinishNotification, this)
    );
    State::get(env).pending_notifications.push_back(
        (MPMoviePlayerPlaybackStateDidChangeNotification, this)
    );
}

- (())stop {
    log!("TODO: [(MPMoviePlayerController*){:?} stop]", this);
    assert!(this == env.framework_state.media_player.movie_player.active_player.take().unwrap());
    
    // Update playback state
    env.objc.borrow_mut::<MPMoviePlayerControllerHostObject>(this).playback_state = MPMoviePlaybackStateStopped;
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
    while let Some(notif) = State::get(env).pending_notifications.pop_front() {
        let (name, object) = notif;
        let name = ns_string::get_static_str(env, name);
        let center: id = msg_class![env; NSNotificationCenter defaultCenter];
        // TODO: should there be some user info attached?
        let _: () = msg![env; center postNotificationName:name object:object];
    }
}
