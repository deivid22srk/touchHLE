/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `MPMediaItem` and related classes.

use crate::dyld::{ConstantExports, HostConstant};
use crate::objc::{id, msg_super, nil, objc_classes, ClassExports, HostObject};
use std::collections::HashMap;

// MPMediaItem property keys (NSString constants)
pub const MPMediaItemPropertyPersistentID: &str = "persistentID";
pub const MPMediaItemPropertyMediaType: &str = "mediaType";
pub const MPMediaItemPropertyTitle: &str = "title";
pub const MPMediaItemPropertyAlbumTitle: &str = "albumTitle";
pub const MPMediaItemPropertyAlbumPersistentID: &str = "albumPersistentID";
pub const MPMediaItemPropertyArtist: &str = "artist";
pub const MPMediaItemPropertyAlbumArtist: &str = "albumArtist";
pub const MPMediaItemPropertyGenre: &str = "genre";
pub const MPMediaItemPropertyComposer: &str = "composer";
pub const MPMediaItemPropertyPlaybackDuration: &str = "playbackDuration";
pub const MPMediaItemPropertyAlbumTrackNumber: &str = "albumTrackNumber";
pub const MPMediaItemPropertyAlbumTrackCount: &str = "albumTrackCount";
pub const MPMediaItemPropertyDiscNumber: &str = "discNumber";
pub const MPMediaItemPropertyDiscCount: &str = "discCount";
pub const MPMediaItemPropertyArtwork: &str = "artwork";
pub const MPMediaItemPropertyLyrics: &str = "lyrics";
pub const MPMediaItemPropertyIsCompilation: &str = "isCompilation";
pub const MPMediaItemPropertyReleaseDate: &str = "releaseDate";
pub const MPMediaItemPropertyBeatsPerMinute: &str = "beatsPerMinute";
pub const MPMediaItemPropertyComments: &str = "comments";
pub const MPMediaItemPropertyAssetURL: &str = "assetURL";
#[allow(dead_code)]
pub const MPMediaItemPropertyIsCloudItem: &str = "isCloudItem";
#[allow(dead_code)]
pub const MPMediaItemPropertyPodcastTitle: &str = "podcastTitle";
#[allow(dead_code)]
pub const MPMediaItemPropertyPlayCount: &str = "playCount";
#[allow(dead_code)]
pub const MPMediaItemPropertySkipCount: &str = "skipCount";
#[allow(dead_code)]
pub const MPMediaItemPropertyRating: &str = "rating";
#[allow(dead_code)]
pub const MPMediaItemPropertyLastPlayedDate: &str = "lastPlayedDate";
#[allow(dead_code)]
pub const MPMediaItemPropertyUserGrouping: &str = "userGrouping";

// iOS 3.1.3 additional properties
#[allow(dead_code)]
pub const MPMediaItemPropertyBookmarkTime: &str = "bookmarkTime";

// Media types (NSNumber values)
#[allow(dead_code)]
pub const MPMediaTypeMusic: u64 = 1 << 0;
#[allow(dead_code)]
pub const MPMediaTypePodcast: u64 = 1 << 1;
#[allow(dead_code)]
pub const MPMediaTypeAudioBook: u64 = 1 << 2;
#[allow(dead_code)]
pub const MPMediaTypeAnyAudio: u64 = 0xFF;

/// Export these constants for dyld
pub const CONSTANTS: ConstantExports = &[
    (
        "_MPMediaItemPropertyPersistentID",
        HostConstant::NSString(MPMediaItemPropertyPersistentID),
    ),
    (
        "_MPMediaItemPropertyMediaType",
        HostConstant::NSString(MPMediaItemPropertyMediaType),
    ),
    (
        "_MPMediaItemPropertyTitle",
        HostConstant::NSString(MPMediaItemPropertyTitle),
    ),
    (
        "_MPMediaItemPropertyAlbumTitle",
        HostConstant::NSString(MPMediaItemPropertyAlbumTitle),
    ),
    (
        "_MPMediaItemPropertyAlbumPersistentID",
        HostConstant::NSString(MPMediaItemPropertyAlbumPersistentID),
    ),
    (
        "_MPMediaItemPropertyArtist",
        HostConstant::NSString(MPMediaItemPropertyArtist),
    ),
    (
        "_MPMediaItemPropertyAlbumArtist",
        HostConstant::NSString(MPMediaItemPropertyAlbumArtist),
    ),
    (
        "_MPMediaItemPropertyGenre",
        HostConstant::NSString(MPMediaItemPropertyGenre),
    ),
    (
        "_MPMediaItemPropertyComposer",
        HostConstant::NSString(MPMediaItemPropertyComposer),
    ),
    (
        "_MPMediaItemPropertyPlaybackDuration",
        HostConstant::NSString(MPMediaItemPropertyPlaybackDuration),
    ),
    (
        "_MPMediaItemPropertyAlbumTrackNumber",
        HostConstant::NSString(MPMediaItemPropertyAlbumTrackNumber),
    ),
    (
        "_MPMediaItemPropertyAlbumTrackCount",
        HostConstant::NSString(MPMediaItemPropertyAlbumTrackCount),
    ),
    (
        "_MPMediaItemPropertyDiscNumber",
        HostConstant::NSString(MPMediaItemPropertyDiscNumber),
    ),
    (
        "_MPMediaItemPropertyDiscCount",
        HostConstant::NSString(MPMediaItemPropertyDiscCount),
    ),
    (
        "_MPMediaItemPropertyArtwork",
        HostConstant::NSString(MPMediaItemPropertyArtwork),
    ),
    (
        "_MPMediaItemPropertyLyrics",
        HostConstant::NSString(MPMediaItemPropertyLyrics),
    ),
    (
        "_MPMediaItemPropertyIsCompilation",
        HostConstant::NSString(MPMediaItemPropertyIsCompilation),
    ),
    (
        "_MPMediaItemPropertyReleaseDate",
        HostConstant::NSString(MPMediaItemPropertyReleaseDate),
    ),
    (
        "_MPMediaItemPropertyBeatsPerMinute",
        HostConstant::NSString(MPMediaItemPropertyBeatsPerMinute),
    ),
    (
        "_MPMediaItemPropertyComments",
        HostConstant::NSString(MPMediaItemPropertyComments),
    ),
    (
        "_MPMediaItemPropertyAssetURL",
        HostConstant::NSString(MPMediaItemPropertyAssetURL),
    ),
];

/// MPMediaItem host object (stub - just stores properties)
struct MPMediaItemHostObject {
    properties: HashMap<String, id>,
}
impl HostObject for MPMediaItemHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation MPMediaItem: NSObject

+ (id)alloc {
    let host_object = Box::new(MPMediaItemHostObject {
        properties: HashMap::new(),
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)valueForProperty:(id)property { // NSString*
    use crate::frameworks::foundation::ns_string::to_rust_string;

    if property == nil {
        return nil;
    }

    let property_str = to_rust_string(env, property);
    let host_obj = env.objc.borrow::<MPMediaItemHostObject>(this);

    if let Some(&value) = host_obj.properties.get(property_str.as_ref()) {
        value
    } else {
        log_dbg!("MPMediaItem: property '{}' not found, returning nil", property_str);
        nil
    }
}

- (())dealloc {
    use crate::objc::release;

    let values_to_release: Vec<_> = {
        let host_obj = env.objc.borrow_mut::<MPMediaItemHostObject>(this);
        host_obj.properties.drain()
            .filter(|(_, value)| *value != nil)
            .map(|(_, value)| value)
            .collect()
    };

    for value in values_to_release {
        release(env, value);
    }

    () = msg_super![env; this dealloc]
}

@end

};
