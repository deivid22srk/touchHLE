/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `MPMediaLibrary`.

use crate::objc::{id, msg, nil, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation MPMediaLibrary: NSObject

+ (id)defaultMediaLibrary {
    use crate::objc::HostObject;
    
    struct MPMediaLibraryHostObject;
    impl HostObject for MPMediaLibraryHostObject {}
    
    log_dbg!("[MPMediaLibrary defaultMediaLibrary] - creating stub media library");
    
    let library: id = msg![env; this alloc];
    let library = if library != nil {
        msg![env; library init]
    } else {
        nil
    };
    library
}

+ (id)alloc {
    use crate::objc::HostObject;
    
    struct MPMediaLibraryHostObject;
    impl HostObject for MPMediaLibraryHostObject {}
    
    let host_object = Box::new(MPMediaLibraryHostObject);
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)init {
    this
}

@end

};
