/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIProgressView`.

use crate::frameworks::core_graphics::{CGFloat, CGRect};
use crate::frameworks::foundation::ns_string::get_static_str;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

type UIProgressViewStyle = NSInteger;
const UIProgressViewStyleDefault: UIProgressViewStyle = 0;
const UIProgressViewStyleBar: UIProgressViewStyle = 1;

#[derive(Default)]
struct UIProgressViewHostObject {
    superclass: super::UIViewHostObject,
    progress: CGFloat,
    progress_view_style: UIProgressViewStyle,
    /// `UIColor*`
    progress_tint_color: id,
    /// `UIColor*`
    track_tint_color: id,
}
impl_HostObject_with_superclass!(UIProgressViewHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIProgressView: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIProgressViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress = 0.0;
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress_view_style = UIProgressViewStyleDefault;
    this
}

- (id)initWithProgressViewStyle:(UIProgressViewStyle)style {
    let frame = CGRect {
        origin: crate::frameworks::core_graphics::CGPoint { x: 0.0, y: 0.0 },
        size: crate::frameworks::core_graphics::CGSize { width: 0.0, height: 0.0 }
    };
    let this: id = msg_super![env; this initWithFrame:frame];
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress_view_style = style;
    this
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];

    // Decode progress value
    let progress_key = get_static_str(env, "UIProgress");
    if msg![env; coder containsValueForKey:progress_key] {
        let progress: CGFloat = msg![env; coder decodeDoubleForKey:progress_key];
        () = msg![env; this setProgress:progress];
    }

    // Decode progress view style
    let style_key = get_static_str(env, "UIProgressViewStyle");
    if msg![env; coder containsValueForKey:style_key] {
        let style: UIProgressViewStyle = msg![env; coder decodeIntForKey:style_key];
        env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress_view_style = style;
    }

    // Decode progress tint color
    let progress_tint_key = get_static_str(env, "UIProgressTintColor");
    let progress_tint: id = msg![env; coder decodeObjectForKey:progress_tint_key];
    if progress_tint != nil {
        () = msg![env; this setProgressTintColor:progress_tint];
    }

    // Decode track tint color
    let track_tint_key = get_static_str(env, "UITrackTintColor");
    let track_tint: id = msg![env; coder decodeObjectForKey:track_tint_key];
    if track_tint != nil {
        () = msg![env; this setTrackTintColor:track_tint];
    }

    this
}

- (())dealloc {
    let &UIProgressViewHostObject {
        superclass: _,
        progress: _,
        progress_view_style: _,
        progress_tint_color,
        track_tint_color,
    } = env.objc.borrow(this);
    release(env, progress_tint_color);
    release(env, track_tint_color);
    msg_super![env; this dealloc]
}

- (CGFloat)progress {
    env.objc.borrow::<UIProgressViewHostObject>(this).progress
}

- (())setProgress:(CGFloat)progress {
    let progress = progress.max(0.0).min(1.0); // Clamp between 0.0 and 1.0
    env.objc.borrow_mut::<UIProgressViewHostObject>(this).progress = progress;
    () = msg![env; this setNeedsDisplay];
}

- (id)progressTintColor {
    env.objc.borrow::<UIProgressViewHostObject>(this).progress_tint_color
}

- (())setProgressTintColor:(id)color { // UIColor*
    let host_obj = env.objc.borrow_mut::<UIProgressViewHostObject>(this);
    let old_color = std::mem::replace(&mut host_obj.progress_tint_color, color);
    retain(env, color);
    release(env, old_color);
}

- (id)trackTintColor {
    env.objc.borrow::<UIProgressViewHostObject>(this).track_tint_color
}

- (())setTrackTintColor:(id)color { // UIColor*
    let host_obj = env.objc.borrow_mut::<UIProgressViewHostObject>(this);
    let old_color = std::mem::replace(&mut host_obj.track_tint_color, color);
    retain(env, color);
    release(env, old_color);
}

@end

};
