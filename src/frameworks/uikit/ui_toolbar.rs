/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIToolbar`.

use super::ui_view::UIViewHostObject;
use crate::frameworks::core_graphics::{CGRect, CGSize};
use crate::frameworks::foundation::ns_array;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

// UIBarStyle (reused from navigation bar)
type UIBarStyle = i32;
#[allow(dead_code)]
const UI_BAR_STYLE_DEFAULT: UIBarStyle = 0;
#[allow(dead_code)]
const UI_BAR_STYLE_BLACK: UIBarStyle = 1;
#[allow(dead_code)]
const UI_BAR_STYLE_BLACK_OPAQUE: UIBarStyle = 1;
#[allow(dead_code)]
const UI_BAR_STYLE_BLACK_TRANSLUCENT: UIBarStyle = 2;

pub struct UIToolbarHostObject {
    superclass: UIViewHostObject,
    bar_style: UIBarStyle,
    items: Vec<id>, // NSArray of UIBarButtonItem
    tint_color: id, // UIColor
    translucent: bool,
}
impl_HostObject_with_superclass!(UIToolbarHostObject);

impl Default for UIToolbarHostObject {
    fn default() -> Self {
        UIToolbarHostObject {
            superclass: Default::default(),
            bar_style: UI_BAR_STYLE_DEFAULT,
            items: Vec::new(),
            tint_color: nil,
            translucent: false,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIToolbar: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIToolbarHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return nil;
    }
    
    // Decode bar style
    let bar_style_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIBarStyle");
    if msg![env; coder containsValueForKey:bar_style_key] {
        let bar_style: i32 = msg![env; coder decodeIntForKey:bar_style_key];
        env.objc.borrow_mut::<UIToolbarHostObject>(this).bar_style = bar_style;
    }
    
    // Decode translucent property
    let translucent_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITranslucent");
    if msg![env; coder containsValueForKey:translucent_key] {
        let translucent: bool = msg![env; coder decodeBoolForKey:translucent_key];
        env.objc.borrow_mut::<UIToolbarHostObject>(this).translucent = translucent;
    }
    
    // Decode tint color
    let tint_color_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITintColor");
    if msg![env; coder containsValueForKey:tint_color_key] {
        let tint_color: id = msg![env; coder decodeObjectForKey:tint_color_key];
        if tint_color != nil {
            retain(env, tint_color);
            env.objc.borrow_mut::<UIToolbarHostObject>(this).tint_color = tint_color;
        }
    }
    
    // Decode items
    let items_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIItems");
    let items: id = msg![env; coder decodeObjectForKey:items_key];
    if items != nil {
        let count: u32 = msg![env; items count];
        let host_obj = env.objc.borrow_mut::<UIToolbarHostObject>(this);
        for i in 0..count {
            let item: id = msg![env; items objectAtIndex:i];
            retain(env, item);
            host_obj.items.push(item);
        }
    }
    
    this
}

- (())setBarStyle:(UIBarStyle)style {
    env.objc.borrow_mut::<UIToolbarHostObject>(this).bar_style = style;
}

- (UIBarStyle)barStyle {
    env.objc.borrow::<UIToolbarHostObject>(this).bar_style
}

- (())setTranslucent:(bool)translucent {
    env.objc.borrow_mut::<UIToolbarHostObject>(this).translucent = translucent;
}

- (bool)isTranslucent {
    env.objc.borrow::<UIToolbarHostObject>(this).translucent
}

- (())setTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UIToolbarHostObject>(this);
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.tint_color = color;
}

- (id)tintColor { // UIColor
    env.objc.borrow::<UIToolbarHostObject>(this).tint_color
}

- (())setItems:(id)items animated:(bool)animated { // NSArray of UIBarButtonItem
    let host_obj = env.objc.borrow_mut::<UIToolbarHostObject>(this);
    
    // Release old items
    for &item in &host_obj.items {
        release(env, item);
    }
    host_obj.items.clear();
    
    if items != nil {
        let count: u32 = msg![env; items count];
        for i in 0..count {
            let item: id = msg![env; items objectAtIndex:i];
            retain(env, item);
            host_obj.items.push(item);
        }
    }
    
    log_dbg!("UIToolbar: set {} items (animated: {})", host_obj.items.len(), animated);
}

- (())setItems:(id)items { // NSArray of UIBarButtonItem
    () = msg![env; this setItems:items animated:false]
}

- (id)items { // NSArray
    let host_obj = env.objc.borrow::<UIToolbarHostObject>(this);
    ns_array::from_vec(env, host_obj.items.clone())
}

- (CGSize)sizeThatFits:(CGSize)size {
    // Standard toolbar height
    CGSize {
        width: size.width,
        height: 44.0,
    }
}

- (())dealloc {
    let host_obj = env.objc.borrow_mut::<UIToolbarHostObject>(this);
    
    for &item in &host_obj.items {
        release(env, item);
    }
    
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
