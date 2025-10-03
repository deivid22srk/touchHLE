/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UINavigationBar` and `UINavigationItem`.

use super::ui_view::UIViewHostObject;
use crate::frameworks::core_graphics::{CGRect, CGSize};
use crate::frameworks::foundation::ns_array;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, HostObject, NSZonePtr,
};

// UIBarStyle
type UIBarStyle = i32;
#[allow(dead_code)]
const UI_BAR_STYLE_DEFAULT: UIBarStyle = 0;
#[allow(dead_code)]
const UI_BAR_STYLE_BLACK: UIBarStyle = 1;
#[allow(dead_code)]
const UI_BAR_STYLE_BLACK_OPAQUE: UIBarStyle = 1;
#[allow(dead_code)]
const UI_BAR_STYLE_BLACK_TRANSLUCENT: UIBarStyle = 2;

pub struct UINavigationBarHostObject {
    superclass: UIViewHostObject,
    bar_style: UIBarStyle,
    items: Vec<id>, // NSArray of UINavigationItem
    delegate: id,   // weak reference
    tint_color: id, // UIColor
    translucent: bool,
}
impl_HostObject_with_superclass!(UINavigationBarHostObject);

impl Default for UINavigationBarHostObject {
    fn default() -> Self {
        UINavigationBarHostObject {
            superclass: Default::default(),
            bar_style: UI_BAR_STYLE_DEFAULT,
            items: Vec::new(),
            delegate: nil,
            tint_color: nil,
            translucent: false,
        }
    }
}

pub struct UINavigationItemHostObject {
    title: id,              // NSString
    title_view: id,         // UIView
    left_bar_button_item: id,  // UIBarButtonItem
    right_bar_button_item: id, // UIBarButtonItem
    back_bar_button_item: id,  // UIBarButtonItem
    #[allow(dead_code)]
    prompt: id,             // NSString
    #[allow(dead_code)]
    hidesBackButton: bool,
}
impl HostObject for UINavigationItemHostObject {}

impl Default for UINavigationItemHostObject {
    fn default() -> Self {
        UINavigationItemHostObject {
            title: nil,
            title_view: nil,
            left_bar_button_item: nil,
            right_bar_button_item: nil,
            back_bar_button_item: nil,
            prompt: nil,
            hidesBackButton: false,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UINavigationBar: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UINavigationBarHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    
    // Set default navigation bar height if not specified
    let host_obj = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    
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
        env.objc.borrow_mut::<UINavigationBarHostObject>(this).bar_style = bar_style;
    }
    
    // Decode translucent property
    let translucent_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITranslucent");
    if msg![env; coder containsValueForKey:translucent_key] {
        let translucent: bool = msg![env; coder decodeBoolForKey:translucent_key];
        env.objc.borrow_mut::<UINavigationBarHostObject>(this).translucent = translucent;
    }
    
    // Decode tint color
    let tint_color_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITintColor");
    if msg![env; coder containsValueForKey:tint_color_key] {
        let tint_color: id = msg![env; coder decodeObjectForKey:tint_color_key];
        if tint_color != nil {
            retain(env, tint_color);
            env.objc.borrow_mut::<UINavigationBarHostObject>(this).tint_color = tint_color;
        }
    }
    
    this
}

- (())setBarStyle:(UIBarStyle)style {
    env.objc.borrow_mut::<UINavigationBarHostObject>(this).bar_style = style;
}

- (UIBarStyle)barStyle {
    env.objc.borrow::<UINavigationBarHostObject>(this).bar_style
}

- (())setTranslucent:(bool)translucent {
    env.objc.borrow_mut::<UINavigationBarHostObject>(this).translucent = translucent;
}

- (bool)isTranslucent {
    env.objc.borrow::<UINavigationBarHostObject>(this).translucent
}

- (())setTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.tint_color = color;
}

- (id)tintColor { // UIColor
    env.objc.borrow::<UINavigationBarHostObject>(this).tint_color
}

- (())setDelegate:(id)delegate {
    env.objc.borrow_mut::<UINavigationBarHostObject>(this).delegate = delegate;
}

- (id)delegate {
    env.objc.borrow::<UINavigationBarHostObject>(this).delegate
}

- (())pushNavigationItem:(id)item animated:(bool)animated {
    if item == nil {
        return;
    }
    retain(env, item);
    env.objc.borrow_mut::<UINavigationBarHostObject>(this).items.push(item);
    log_dbg!("UINavigationBar: pushed navigation item {:?} (animated: {})", item, animated);
}

- (id)popNavigationItemAnimated:(bool)animated {
    let host_obj = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    if let Some(item) = host_obj.items.pop() {
        log_dbg!("UINavigationBar: popped navigation item {:?} (animated: {})", item, animated);
        item
    } else {
        nil
    }
}

- (())setItems:(id)items animated:(bool)animated { // NSArray of UINavigationItem
    let host_obj = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    
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
    
    log_dbg!("UINavigationBar: set {} items (animated: {})", host_obj.items.len(), animated);
}

- (id)items { // NSArray
    let host_obj = env.objc.borrow::<UINavigationBarHostObject>(this);
    ns_array::from_vec(env, host_obj.items.clone())
}

- (id)topItem { // UINavigationItem
    let host_obj = env.objc.borrow::<UINavigationBarHostObject>(this);
    host_obj.items.last().copied().unwrap_or(nil)
}

- (id)backItem { // UINavigationItem
    let host_obj = env.objc.borrow::<UINavigationBarHostObject>(this);
    let len = host_obj.items.len();
    if len >= 2 {
        host_obj.items[len - 2]
    } else {
        nil
    }
}

- (CGSize)sizeThatFits:(CGSize)size {
    // Standard navigation bar height
    CGSize {
        width: size.width,
        height: 44.0,
    }
}

- (())dealloc {
    let host_obj = env.objc.borrow_mut::<UINavigationBarHostObject>(this);
    
    for &item in &host_obj.items {
        release(env, item);
    }
    
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    
    () = msg_super![env; this dealloc]
}

@end

@implementation UINavigationItem: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UINavigationItemHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithTitle:(id)title { // NSString
    let this: id = msg_super![env; this init];
    if this != nil && title != nil {
        retain(env, title);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).title = title;
    }
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this init];
    if this == nil {
        return nil;
    }
    
    // Decode title
    let title_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITitle");
    let title: id = msg![env; coder decodeObjectForKey:title_key];
    if title != nil {
        retain(env, title);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).title = title;
    }
    
    // Decode title view
    let title_view_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITitleView");
    let title_view: id = msg![env; coder decodeObjectForKey:title_view_key];
    if title_view != nil {
        retain(env, title_view);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).title_view = title_view;
    }
    
    // Decode bar button items
    let left_item_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UILeftBarButtonItem");
    let left_item: id = msg![env; coder decodeObjectForKey:left_item_key];
    if left_item != nil {
        retain(env, left_item);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).left_bar_button_item = left_item;
    }
    
    let right_item_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIRightBarButtonItem");
    let right_item: id = msg![env; coder decodeObjectForKey:right_item_key];
    if right_item != nil {
        retain(env, right_item);
        env.objc.borrow_mut::<UINavigationItemHostObject>(this).right_bar_button_item = right_item;
    }
    
    this
}

- (())setTitle:(id)title { // NSString
    let host_obj = env.objc.borrow_mut::<UINavigationItemHostObject>(this);
    if host_obj.title != nil {
        release(env, host_obj.title);
    }
    if title != nil {
        retain(env, title);
    }
    host_obj.title = title;
}

- (id)title { // NSString
    env.objc.borrow::<UINavigationItemHostObject>(this).title
}

- (())setTitleView:(id)view { // UIView
    let host_obj = env.objc.borrow_mut::<UINavigationItemHostObject>(this);
    if host_obj.title_view != nil {
        release(env, host_obj.title_view);
    }
    if view != nil {
        retain(env, view);
    }
    host_obj.title_view = view;
}

- (id)titleView { // UIView
    env.objc.borrow::<UINavigationItemHostObject>(this).title_view
}

- (())setLeftBarButtonItem:(id)item { // UIBarButtonItem
    let host_obj = env.objc.borrow_mut::<UINavigationItemHostObject>(this);
    if host_obj.left_bar_button_item != nil {
        release(env, host_obj.left_bar_button_item);
    }
    if item != nil {
        retain(env, item);
    }
    host_obj.left_bar_button_item = item;
}

- (id)leftBarButtonItem { // UIBarButtonItem
    env.objc.borrow::<UINavigationItemHostObject>(this).left_bar_button_item
}

- (())setRightBarButtonItem:(id)item { // UIBarButtonItem
    let host_obj = env.objc.borrow_mut::<UINavigationItemHostObject>(this);
    if host_obj.right_bar_button_item != nil {
        release(env, host_obj.right_bar_button_item);
    }
    if item != nil {
        retain(env, item);
    }
    host_obj.right_bar_button_item = item;
}

- (id)rightBarButtonItem { // UIBarButtonItem
    env.objc.borrow::<UINavigationItemHostObject>(this).right_bar_button_item
}

- (())setBackBarButtonItem:(id)item { // UIBarButtonItem
    let host_obj = env.objc.borrow_mut::<UINavigationItemHostObject>(this);
    if host_obj.back_bar_button_item != nil {
        release(env, host_obj.back_bar_button_item);
    }
    if item != nil {
        retain(env, item);
    }
    host_obj.back_bar_button_item = item;
}

- (id)backBarButtonItem { // UIBarButtonItem
    env.objc.borrow::<UINavigationItemHostObject>(this).back_bar_button_item
}

- (())dealloc {
    let host_obj = env.objc.borrow::<UINavigationItemHostObject>(this);
    
    if host_obj.title != nil {
        release(env, host_obj.title);
    }
    if host_obj.title_view != nil {
        release(env, host_obj.title_view);
    }
    if host_obj.left_bar_button_item != nil {
        release(env, host_obj.left_bar_button_item);
    }
    if host_obj.right_bar_button_item != nil {
        release(env, host_obj.right_bar_button_item);
    }
    if host_obj.back_bar_button_item != nil {
        release(env, host_obj.back_bar_button_item);
    }
    
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
