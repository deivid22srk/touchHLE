/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIProgressView`.

use super::ui_view::UIViewHostObject;
use crate::frameworks::core_graphics::CGRect;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

// UIProgressViewStyle
type UIProgressViewStyle = NSInteger;
#[allow(dead_code)]
const UI_PROGRESS_VIEW_STYLE_DEFAULT: UIProgressViewStyle = 0;
#[allow(dead_code)]
const UI_PROGRESS_VIEW_STYLE_BAR: UIProgressViewStyle = 1;

pub struct UIProgressViewHostObject {
    superclass: UIViewHostObject,
    style: UIProgressViewStyle,
    progress: f32,
    progress_tint_color: id,  // UIColor
    track_tint_color: id,     // UIColor
}
impl_HostObject_with_superclass!(UIProgressViewHostObject);

impl Default for UIProgressViewHostObject {
    fn default() -> Self {
        UIProgressViewHostObject {
            superclass: Default::default(),
            style: UI_PROGRESS_VIEW_STYLE_DEFAULT,
            progress: 0.0,
            progress_tint_color: nil,
            track_tint_color: nil,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIProgressView: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIProgressViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithProgressViewStyle:(UIProgressViewStyle)style {
    let frame = CGRect {
        origin: crate::frameworks::core_graphics::CGPoint { x: 0.0, y: 0.0 },
        size: crate::frameworks::core_graphics::CGSize { width: 150.0, height: 9.0 },
    };
    let this: id = msg_super![env; this initWithFrame:frame];
    if this != nil {
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).style = style;
    }
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return this;
    }
    
    // Decode style
    let style_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIProgressViewStyle");
    if msg![env; coder containsValueForKey:style_key] {
        let style: NSInteger = msg![env; coder decodeIntegerForKey:style_key];
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).style = style;
    }
    
    // Decode progress
    let progress_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIProgress");
    if msg![env; coder containsValueForKey:progress_key] {
        let progress: f32 = msg![env; coder decodeFloatForKey:progress_key];
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress = progress;
    }
    
    // Decode progress tint color
    let progress_tint_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIProgressTintColor");
    let progress_tint: id = msg![env; coder decodeObjectForKey:progress_tint_key];
    if progress_tint != nil {
        retain(env, progress_tint);
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress_tint_color = progress_tint;
    }
    
    // Decode track tint color
    let track_tint_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITrackTintColor");
    let track_tint: id = msg![env; coder decodeObjectForKey:track_tint_key];
    if track_tint != nil {
        retain(env, track_tint);
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).track_tint_color = track_tint;
    }
    
    this
}

- (())setProgressViewStyle:(UIProgressViewStyle)style {
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).style = style;
}

- (UIProgressViewStyle)progressViewStyle {
    env.objc.borrow::<UIProgressViewHostObject>(this).style
}

- (())setProgress:(f32)progress {
    let clamped = progress.clamp(0.0, 1.0);
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress = clamped;
    log_dbg!("UIProgressView {:?} setProgress: {}", this, clamped);
}

- (f32)progress {
    env.objc.borrow::<UIProgressViewHostObject>(this).progress
}

- (())setProgress:(f32)progress animated:(bool)_animated {
    let clamped = progress.clamp(0.0, 1.0);
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress = clamped;
    log_dbg!("UIProgressView {:?} setProgress:animated: {} (animated ignored)", this, clamped);
}

- (())setProgressTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UIProgressViewHostObject>(this);
    if host_obj.progress_tint_color != nil {
        release(env, host_obj.progress_tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.progress_tint_color = color;
}

- (id)progressTintColor { // UIColor
    env.objc.borrow::<UIProgressViewHostObject>(this).progress_tint_color
}

- (())setTrackTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UIProgressViewHostObject>(this);
    if host_obj.track_tint_color != nil {
        release(env, host_obj.track_tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.track_tint_color = color;
}

- (id)trackTintColor { // UIColor
    env.objc.borrow::<UIProgressViewHostObject>(this).track_tint_color
}

- (())dealloc {
    let host_obj = env.objc.borrow::<UIProgressViewHostObject>(this);
    
    if host_obj.progress_tint_color != nil {
        release(env, host_obj.progress_tint_color);
    }
    if host_obj.track_tint_color != nil {
        release(env, host_obj.track_tint_color);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
