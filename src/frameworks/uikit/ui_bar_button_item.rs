/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIBarButtonItem`.

use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, msg_super, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr, SEL,
};

// UIBarButtonItemStyle
type UIBarButtonItemStyle = NSInteger;
#[allow(dead_code)]
const UI_BAR_BUTTON_ITEM_STYLE_PLAIN: UIBarButtonItemStyle = 0;
#[allow(dead_code)]
const UI_BAR_BUTTON_ITEM_STYLE_BORDERED: UIBarButtonItemStyle = 1;
#[allow(dead_code)]
const UI_BAR_BUTTON_ITEM_STYLE_DONE: UIBarButtonItemStyle = 2;

// UIBarButtonSystemItem
type UIBarButtonSystemItem = NSInteger;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_DONE: UIBarButtonSystemItem = 0;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_CANCEL: UIBarButtonSystemItem = 1;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_EDIT: UIBarButtonSystemItem = 2;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_SAVE: UIBarButtonSystemItem = 3;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_ADD: UIBarButtonSystemItem = 4;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_FLEXIBLE_SPACE: UIBarButtonSystemItem = 5;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_FIXED_SPACE: UIBarButtonSystemItem = 6;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_COMPOSE: UIBarButtonSystemItem = 7;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_REPLY: UIBarButtonSystemItem = 8;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_ACTION: UIBarButtonSystemItem = 9;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_ORGANIZE: UIBarButtonSystemItem = 10;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_BOOKMARKS: UIBarButtonSystemItem = 11;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_SEARCH: UIBarButtonSystemItem = 12;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_REFRESH: UIBarButtonSystemItem = 13;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_STOP: UIBarButtonSystemItem = 14;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_CAMERA: UIBarButtonSystemItem = 15;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_TRASH: UIBarButtonSystemItem = 16;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_PLAY: UIBarButtonSystemItem = 17;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_PAUSE: UIBarButtonSystemItem = 18;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_REWIND: UIBarButtonSystemItem = 19;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_FAST_FORWARD: UIBarButtonSystemItem = 20;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_UNDO: UIBarButtonSystemItem = 21;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_REDO: UIBarButtonSystemItem = 22;
#[allow(dead_code)]
const UI_BAR_BUTTON_SYSTEM_ITEM_PAGE_CURL: UIBarButtonSystemItem = 23;

pub struct UIBarButtonItemHostObject {
    title: id,          // NSString
    image: id,          // UIImage
    custom_view: id,    // UIView
    target: id,         // weak reference
    action: Option<SEL>,
    style: UIBarButtonItemStyle,
    width: f32,
    enabled: bool,
    tint_color: id,     // UIColor
    #[allow(dead_code)]
    system_item: Option<UIBarButtonSystemItem>,
}
impl HostObject for UIBarButtonItemHostObject {}

impl Default for UIBarButtonItemHostObject {
    fn default() -> Self {
        UIBarButtonItemHostObject {
            title: nil,
            image: nil,
            custom_view: nil,
            target: nil,
            action: None,
            style: UI_BAR_BUTTON_ITEM_STYLE_PLAIN,
            width: 0.0,
            enabled: true,
            tint_color: nil,
            system_item: None,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIBarButtonItem: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIBarButtonItemHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithTitle:(id)title // NSString
              style:(UIBarButtonItemStyle)style
             target:(id)target
             action:(SEL)action {
    let this: id = msg_super![env; this init];
    if this != nil {
        if title != nil {
            retain(env, title);
        }
        let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
        host_obj.title = title;
        host_obj.style = style;
        host_obj.target = target;
        host_obj.action = if action.is_null() { None } else { Some(action) };
    }
    this
}

- (id)initWithImage:(id)image // UIImage
              style:(UIBarButtonItemStyle)style
             target:(id)target
             action:(SEL)action {
    let this: id = msg_super![env; this init];
    if this != nil {
        if image != nil {
            retain(env, image);
        }
        let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
        host_obj.image = image;
        host_obj.style = style;
        host_obj.target = target;
        host_obj.action = if action.is_null() { None } else { Some(action) };
    }
    this
}

- (id)initWithBarButtonSystemItem:(UIBarButtonSystemItem)system_item
                           target:(id)target
                           action:(SEL)action {
    let this: id = msg_super![env; this init];
    if this != nil {
        let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
        host_obj.system_item = Some(system_item);
        host_obj.target = target;
        host_obj.action = if action.is_null() { None } else { Some(action) };
        
        // Set default title based on system item
        use crate::frameworks::foundation::ns_string::from_rust_string;
        let title_str = match system_item {
            UI_BAR_BUTTON_SYSTEM_ITEM_DONE => "Done",
            UI_BAR_BUTTON_SYSTEM_ITEM_CANCEL => "Cancel",
            UI_BAR_BUTTON_SYSTEM_ITEM_EDIT => "Edit",
            UI_BAR_BUTTON_SYSTEM_ITEM_SAVE => "Save",
            UI_BAR_BUTTON_SYSTEM_ITEM_ADD => "+",
            UI_BAR_BUTTON_SYSTEM_ITEM_UNDO => "Undo",
            UI_BAR_BUTTON_SYSTEM_ITEM_REDO => "Redo",
            _ => "",
        };
        if !title_str.is_empty() {
            let title = from_rust_string(env, title_str.to_string());
            host_obj.title = title;
        }
    }
    this
}

- (id)initWithCustomView:(id)custom_view { // UIView
    let this: id = msg_super![env; this init];
    if this != nil && custom_view != nil {
        retain(env, custom_view);
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).custom_view = custom_view;
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
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).title = title;
    }
    
    // Decode image
    let image_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIImage");
    let image: id = msg![env; coder decodeObjectForKey:image_key];
    if image != nil {
        retain(env, image);
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).image = image;
    }
    
    // Decode custom view
    let custom_view_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UICustomView");
    let custom_view: id = msg![env; coder decodeObjectForKey:custom_view_key];
    if custom_view != nil {
        retain(env, custom_view);
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).custom_view = custom_view;
    }
    
    // Decode style
    let style_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIStyle");
    if msg![env; coder containsValueForKey:style_key] {
        let style: NSInteger = msg![env; coder decodeIntegerForKey:style_key];
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).style = style;
    }
    
    // Decode width
    let width_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIWidth");
    if msg![env; coder containsValueForKey:width_key] {
        let width: f32 = msg![env; coder decodeFloatForKey:width_key];
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).width = width;
    }
    
    // Decode enabled
    let enabled_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIEnabled");
    if msg![env; coder containsValueForKey:enabled_key] {
        let enabled: bool = msg![env; coder decodeBoolForKey:enabled_key];
        env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).enabled = enabled;
    }
    
    this
}

- (())setTitle:(id)title { // NSString
    let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
    if host_obj.title != nil {
        release(env, host_obj.title);
    }
    if title != nil {
        retain(env, title);
    }
    host_obj.title = title;
}

- (id)title { // NSString
    env.objc.borrow::<UIBarButtonItemHostObject>(this).title
}

- (())setImage:(id)image { // UIImage
    let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
    if host_obj.image != nil {
        release(env, host_obj.image);
    }
    if image != nil {
        retain(env, image);
    }
    host_obj.image = image;
}

- (id)image { // UIImage
    env.objc.borrow::<UIBarButtonItemHostObject>(this).image
}

- (())setCustomView:(id)view { // UIView
    let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
    if host_obj.custom_view != nil {
        release(env, host_obj.custom_view);
    }
    if view != nil {
        retain(env, view);
    }
    host_obj.custom_view = view;
}

- (id)customView { // UIView
    env.objc.borrow::<UIBarButtonItemHostObject>(this).custom_view
}

- (())setTarget:(id)target {
    env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).target = target;
}

- (id)target {
    env.objc.borrow::<UIBarButtonItemHostObject>(this).target
}

- (())setAction:(SEL)action {
    let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
    host_obj.action = if action.is_null() { None } else { Some(action) };
}

- (SEL)action {
    env.objc.borrow::<UIBarButtonItemHostObject>(this).action.unwrap_or(std::ptr::null())
}

- (())setStyle:(UIBarButtonItemStyle)style {
    env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).style = style;
}

- (UIBarButtonItemStyle)style {
    env.objc.borrow::<UIBarButtonItemHostObject>(this).style
}

- (())setWidth:(f32)width {
    env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).width = width;
}

- (f32)width {
    env.objc.borrow::<UIBarButtonItemHostObject>(this).width
}

- (())setEnabled:(bool)enabled {
    env.objc.borrow_mut::<UIBarButtonItemHostObject>(this).enabled = enabled;
}

- (bool)isEnabled {
    env.objc.borrow::<UIBarButtonItemHostObject>(this).enabled
}

- (())setTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UIBarButtonItemHostObject>(this);
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.tint_color = color;
}

- (id)tintColor { // UIColor
    env.objc.borrow::<UIBarButtonItemHostObject>(this).tint_color
}

- (())dealloc {
    let host_obj = env.objc.borrow::<UIBarButtonItemHostObject>(this);
    
    if host_obj.title != nil {
        release(env, host_obj.title);
    }
    if host_obj.image != nil {
        release(env, host_obj.image);
    }
    if host_obj.custom_view != nil {
        release(env, host_obj.custom_view);
    }
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
