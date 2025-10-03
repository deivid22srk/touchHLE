/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIToolbar` and `UIBarButtonItem`.

use crate::frameworks::core_graphics::CGRect;
use crate::frameworks::foundation::ns_string::get_static_str;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

type UIBarStyle = NSInteger;
const UIBarStyleDefault: UIBarStyle = 0;
#[allow(dead_code)]
const UIBarStyleBlack: UIBarStyle = 1;

#[derive(Default)]
struct UIToolbarHostObject {
    superclass: super::UIViewHostObject,
    /// `NSArray*` of UIBarButtonItem
    items: id,
    /// `UIColor*`
    tint_color: id,
    bar_style: UIBarStyle,
}
impl_HostObject_with_superclass!(UIToolbarHostObject);

struct UIBarButtonItemHostObject {
    /// `id` target (weak reference)
    target: id,
    /// `SEL` action
    action: Option<crate::objc::SEL>,
    /// `NSString*` title
    title: id,
    /// `UIImage*` image
    image: id,
}
impl crate::objc::HostObject for UIBarButtonItemHostObject {}
impl Default for UIBarButtonItemHostObject {
    fn default() -> Self {
        UIBarButtonItemHostObject {
            target: nil,
            action: None,
            title: nil,
            image: nil,
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
    // Default styling
    env.objc.borrow_mut::<UIToolbarHostObject>(this).bar_style = UIBarStyleDefault;
    this
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];

    // Decode bar style
    let bar_style_key = get_static_str(env, "UIBarStyle");
    if msg![env; coder containsValueForKey:bar_style_key] {
        let bar_style: UIBarStyle = msg![env; coder decodeIntForKey:bar_style_key];
        () = msg![env; this setBarStyle:bar_style];
    }

    // Decode tint color
    let tint_color_key = get_static_str(env, "UITintColor");
    let tint_color: id = msg![env; coder decodeObjectForKey:tint_color_key];
    if tint_color != nil {
        () = msg![env; this setTintColor:tint_color];
    }

    // Decode items
    let items_key = get_static_str(env, "UIItems");
    let items: id = msg![env; coder decodeObjectForKey:items_key];
    if items != nil {
        () = msg![env; this setItems:items];
    }

    this
}

- (())dealloc {
    let &UIToolbarHostObject {
        superclass: _,
        items,
        tint_color,
        bar_style: _,
    } = env.objc.borrow(this);
    release(env, items);
    release(env, tint_color);
    msg_super![env; this dealloc]
}

- (id)items {
    env.objc.borrow::<UIToolbarHostObject>(this).items
}

- (())setItems:(id)items { // NSArray*
    let host_obj = env.objc.borrow_mut::<UIToolbarHostObject>(this);
    let old_items = std::mem::replace(&mut host_obj.items, items);
    retain(env, items);
    release(env, old_items);
    // TODO: Layout items
}

- (id)tintColor {
    env.objc.borrow::<UIToolbarHostObject>(this).tint_color
}

- (())setTintColor:(id)color { // UIColor*
    let host_obj = env.objc.borrow_mut::<UIToolbarHostObject>(this);
    let old_color = std::mem::replace(&mut host_obj.tint_color, color);
    retain(env, color);
    release(env, old_color);
}

- (UIBarStyle)barStyle {
    env.objc.borrow::<UIToolbarHostObject>(this).bar_style
}

- (())setBarStyle:(UIBarStyle)style {
    env.objc.borrow_mut::<UIToolbarHostObject>(this).bar_style = style;
}

@end

@implementation UIBarButtonItem: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIBarButtonItemHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithTitle:(id)title // NSString*
             style:(NSInteger)_style
            target:(id)target
            action:(crate::objc::SEL)action {
    retain(env, title);
    let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
    host_obj.title = title;
    host_obj.target = target; // weak reference
    host_obj.action = Some(action);
    this
}

- (id)initWithImage:(id)image // UIImage*
             style:(NSInteger)_style
            target:(id)target
            action:(crate::objc::SEL)action {
    retain(env, image);
    let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
    host_obj.image = image;
    host_obj.target = target; // weak reference
    host_obj.action = Some(action);
    this
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    let title_key = get_static_str(env, "UITitle");
    let title: id = msg![env; coder decodeObjectForKey:title_key];

    let image_key = get_static_str(env, "UIImage");
    let image: id = msg![env; coder decodeObjectForKey:image_key];

    if title != nil {
        retain(env, title);
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).title = title;
    }

    if image != nil {
        retain(env, image);
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).image = image;
    }

    // Target and action are typically connected via UIRuntimeEventConnection
    this
}

- (())dealloc {
    let &UIBarButtonItemHostObject {
        target: _,
        action: _,
        title,
        image,
    } = env.objc.borrow(this);
    release(env, title);
    release(env, image);
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
