/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIActivityIndicatorView`.

use super::ui_view::UIViewHostObject;
use crate::frameworks::core_graphics::CGRect;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, objc_classes, ClassExports, NSZonePtr,
};

type UIActivityIndicatorViewStyle = NSInteger;
#[allow(dead_code)]
const UI_ACTIVITY_INDICATOR_VIEW_STYLE_WHITE_LARGE: UIActivityIndicatorViewStyle = 0;
#[allow(dead_code)]
const UI_ACTIVITY_INDICATOR_VIEW_STYLE_WHITE: UIActivityIndicatorViewStyle = 1;
#[allow(dead_code)]
const UI_ACTIVITY_INDICATOR_VIEW_STYLE_GRAY: UIActivityIndicatorViewStyle = 2;

pub struct UIActivityIndicatorViewHostObject {
    superclass: UIViewHostObject,
    style: UIActivityIndicatorViewStyle,
    animating: bool,
    hides_when_stopped: bool,
}
impl_HostObject_with_superclass!(UIActivityIndicatorViewHostObject);

impl Default for UIActivityIndicatorViewHostObject {
    fn default() -> Self {
        UIActivityIndicatorViewHostObject {
            superclass: Default::default(),
            style: UI_ACTIVITY_INDICATOR_VIEW_STYLE_WHITE,
            animating: false,
            hides_when_stopped: true,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIActivityIndicatorView: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIActivityIndicatorViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithActivityIndicatorStyle:(UIActivityIndicatorViewStyle)style {
    let frame = CGRect {
        origin: crate::frameworks::core_graphics::CGPoint { x: 0.0, y: 0.0 },
        size: crate::frameworks::core_graphics::CGSize { width: 20.0, height: 20.0 },
    };
    let this: id = msg_super![env; this initWithFrame:frame];
    if this != nil {
        env.objc.borrow_mut::<UIActivityIndicatorViewHostObject>(this).style = style;
    }
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return this;
    }
    
    // Decode style
    let style_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIActivityIndicatorViewStyle");
    if msg![env; coder containsValueForKey:style_key] {
        let style: NSInteger = msg![env; coder decodeIntegerForKey:style_key];
        env.objc.borrow_mut::<UIActivityIndicatorViewHostObject>(this).style = style;
    }
    
    // Decode hides when stopped
    let hides_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIHidesWhenStopped");
    if msg![env; coder containsValueForKey:hides_key] {
        let hides: bool = msg![env; coder decodeBoolForKey:hides_key];
        env.objc.borrow_mut::<UIActivityIndicatorViewHostObject>(this).hides_when_stopped = hides;
    }
    
    // Decode animating
    let animating_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIAnimating");
    if msg![env; coder containsValueForKey:animating_key] {
        let animating: bool = msg![env; coder decodeBoolForKey:animating_key];
        env.objc.borrow_mut::<UIActivityIndicatorViewHostObject>(this).animating = animating;
    }
    
    this
}

- (())setActivityIndicatorViewStyle:(UIActivityIndicatorViewStyle)style {
    env.objc.borrow_mut::<UIActivityIndicatorViewHostObject>(this).style = style;
}

- (UIActivityIndicatorViewStyle)activityIndicatorViewStyle {
    env.objc.borrow::<UIActivityIndicatorViewHostObject>(this).style
}

- (())startAnimating {
    log_dbg!("UIActivityIndicatorView {:?} startAnimating", this);
    env.objc.borrow_mut::<UIActivityIndicatorViewHostObject>(this).animating = true;
}

- (())stopAnimating {
    log_dbg!("UIActivityIndicatorView {:?} stopAnimating", this);
    env.objc.borrow_mut::<UIActivityIndicatorViewHostObject>(this).animating = false;
}

- (bool)isAnimating {
    env.objc.borrow::<UIActivityIndicatorViewHostObject>(this).animating
}

- (())setHidesWhenStopped:(bool)hides {
    env.objc.borrow_mut::<UIActivityIndicatorViewHostObject>(this).hides_when_stopped = hides;
}

- (bool)hidesWhenStopped {
    env.objc.borrow::<UIActivityIndicatorViewHostObject>(this).hides_when_stopped
}

@end

};
