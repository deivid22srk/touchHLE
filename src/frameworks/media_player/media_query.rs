/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `MPMediaQuery`.

use crate::dyld::{ConstantExports, HostConstant};
use crate::objc::{id, msg, msg_class, nil, objc_classes, ClassExports};

// MPMediaPlaylist property keys (iOS 3.1.3)
pub const MPMediaPlaylistPropertyPersistentID: &str = "playlistPersistentID";
pub const MPMediaPlaylistPropertyName: &str = "playlistName";
pub const MPMediaPlaylistPropertyPlaylistAttributes: &str = "playlistAttributes";
pub const MPMediaPlaylistPropertySeedItems: &str = "seedItems";

/// Export these constants for dyld
pub const CONSTANTS: ConstantExports = &[
    (
        "_MPMediaPlaylistPropertyPersistentID",
        HostConstant::NSString(MPMediaPlaylistPropertyPersistentID),
    ),
    (
        "_MPMediaPlaylistPropertyName",
        HostConstant::NSString(MPMediaPlaylistPropertyName),
    ),
    (
        "_MPMediaPlaylistPropertyPlaylistAttributes",
        HostConstant::NSString(MPMediaPlaylistPropertyPlaylistAttributes),
    ),
    (
        "_MPMediaPlaylistPropertySeedItems",
        HostConstant::NSString(MPMediaPlaylistPropertySeedItems),
    ),
];

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation MPMediaQuery: NSObject

+ (id)alloc {
    use crate::objc::HostObject;
    struct MPMediaQueryHostObject;
    impl HostObject for MPMediaQueryHostObject {}
    
    let host_object = Box::new(MPMediaQueryHostObject);
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)playlistsQuery {
    log_dbg!("[MPMediaQuery playlistsQuery] - returning empty query");
    let query: id = msg![env; this alloc];
    msg![env; query init]
}

+ (id)songsQuery {
    log_dbg!("[MPMediaQuery songsQuery] - returning empty query");
    let query: id = msg![env; this alloc];
    msg![env; query init]
}

+ (id)albumsQuery {
    log_dbg!("[MPMediaQuery albumsQuery] - returning empty query");
    let query: id = msg![env; this alloc];
    msg![env; query init]
}

+ (id)artistsQuery {
    log_dbg!("[MPMediaQuery artistsQuery] - returning empty query");
    let query: id = msg![env; this alloc];
    msg![env; query init]
}

- (id)init {
    this
}

- (id)items {
    use crate::objc::msg_class;
    // Return empty array - no media items available in emulator
    msg_class![env; NSArray array]
}

- (id)collections {
    use crate::objc::msg_class;
    // Return empty array - no media collections available in emulator
    msg_class![env; NSArray array]
}

@end

@implementation MPMediaPlaylist: NSObject

- (id)items {
    use crate::objc::msg_class;
    // Return empty array - no media items in playlist
    msg_class![env; NSArray array]
}

- (id)valueForProperty:(id)property { // NSString*
    log_dbg!("[MPMediaPlaylist valueForProperty:] - returning nil for all properties");
    nil
}

@end

};
