/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UINavigationBar` and `UINavigationItem`.

use crate::frameworks::core_graphics::CGRect;
use crate::frameworks::foundation::ns_string::get_static_str;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

type UIBarStyle = NSInteger;

#[derive(Default)]
struct UINavigationBarHostObject {
    superclass: super::UIViewHostObject,
    /// `NSArray*` of UINavigationItem
    items: id,
    /// `UIColor*`
    tint_color: id,
    bar_style: UIBarStyle,
}
impl_HostObject_with_superclass!(UINavigationBarHostObject);

#[derive(Default)]
struct UINavigationItemHostObject {
    /// `NSString*` title
    title: id,
    /// `UIView*` custom title view
    title_view: id,
    /// `UIBarButtonItem*` left button
    left_button_item: id,
    /// `UIBarButtonItem*` right button
    right_button_item: id,
}
impl crate::objc::HostObject for UINavigationItemHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UINavigationBar: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UINavigationBarHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    env.objc.borrow_mut::<UINavigationBarHostObject>(this).bar_style = 0; // UIBarStyleDefault
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
        retain(env, items);
        env.objc.borrow_mut::<UINavigationBarHostObject>(this).items = items;
    }

    this
}

- (())dealloc {
    let &UINavigationBarHostObject {
        superclass: _,
        items,
        tint_color,
        bar_style: _,
    } = env.objc.borrow(this);
    release(env, items);
    release(env, tint_color);
    msg_super![env; this dealloc]
}

- (id)tintColor {
    env.objc.borrow::<UINavigationBarHostObject>(this).tint_color
}

- (())setTintColor:(id)color { // UIColor*
    let host_obj = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    let old_color = std::mem::replace(&mut host_obj.tint_color, color);
    retain(env, color);
    release(env, old_color);
}

- (UIBarStyle)barStyle {
    env.objc.borrow::<UINavigationBarHostObject>(this).bar_style
}

- (())setBarStyle:(UIBarStyle)style {
    env.objc.borrow_mut::<UINavigationBarHostObject>(this).bar_style = style;
}

- (())pushNavigationItem:(id)item { // UINavigationItem*
    log!("TODO: [(UINavigationBar*) {:?} pushNavigationItem:{:?}]", this, item);
}

@end

@implementation UINavigationItem: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UINavigationItemHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithTitle:(id)title { // NSString*
    retain(env, title);
    env.objc.borrow_mut::<UINavigationItemHostObject>(this).title = title;
    this
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    let title_key = get_static_str(env, "UITitle");
    let title: id = msg![env; coder decodeObjectForKey:title_key];
    if title != nil {
        retain(env, title);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).title = title;
    }

    let title_view_key = get_static_str(env, "UITitleView");
    let title_view: id = msg![env; coder decodeObjectForKey:title_view_key];
    if title_view != nil {
        retain(env, title_view);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).title_view = title_view;
    }

    let left_button_key = get_static_str(env, "UILeftBarButtonItem");
    let left_button: id = msg![env; coder decodeObjectForKey:left_button_key];
    if left_button != nil {
        retain(env, left_button);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).left_button_item = left_button;
    }

    let right_button_key = get_static_str(env, "UIRightBarButtonItem");
    let right_button: id = msg![env; coder decodeObjectForKey:right_button_key];
    if right_button != nil {
        retain(env, right_button);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).right_button_item = right_button;
    }

    this
}

- (())dealloc {
    let &UINavigationItemHostObject {
        title,
        title_view,
        left_button_item,
        right_button_item,
    } = env.objc.borrow(this);
    release(env, title);
    release(env, title_view);
    release(env, left_button_item);
    release(env, right_button_item);
    env.objc.dealloc_object(this, &mut env.mem)
}

- (id)title {
    env.objc.borrow::<UINavigationItemHostObject>(this).title
}

- (())setTitle:(id)title { // NSString*
    let host_obj = env.objc.borrow_mut::<UINavigationItemHostObject>(this);
    let old_title = std::mem::replace(&mut host_obj.title, title);
    retain(env, title);
    release(env, old_title);
}

@end

};
