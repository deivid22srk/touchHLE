/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIActionSheet`.

use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, msg_super, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr,
};

// UIActionSheetStyle
type UIActionSheetStyle = NSInteger;
#[allow(dead_code)]
const UI_ACTION_SHEET_STYLE_AUTOMATIC: UIActionSheetStyle = -1;
#[allow(dead_code)]
const UI_ACTION_SHEET_STYLE_DEFAULT: UIActionSheetStyle = 0;
#[allow(dead_code)]
const UI_ACTION_SHEET_STYLE_BLACK_TRANSLUCENT: UIActionSheetStyle = 1;
#[allow(dead_code)]
const UI_ACTION_SHEET_STYLE_BLACK_OPAQUE: UIActionSheetStyle = 2;

pub struct UIActionSheetHostObject {
    title: id,                  // NSString
    delegate: id,               // weak reference
    button_titles: Vec<id>,     // Array of NSString
    cancel_button_index: NSInteger,
    destructive_button_index: NSInteger,
    first_other_button_index: NSInteger,
    action_sheet_style: UIActionSheetStyle,
    visible: bool,
}
impl HostObject for UIActionSheetHostObject {}

impl Default for UIActionSheetHostObject {
    fn default() -> Self {
        UIActionSheetHostObject {
            title: nil,
            delegate: nil,
            button_titles: Vec::new(),
            cancel_button_index: -1,
            destructive_button_index: -1,
            first_other_button_index: -1,
            action_sheet_style: UI_ACTION_SHEET_STYLE_AUTOMATIC,
            visible: false,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIActionSheet: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIActionSheetHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithTitle:(id)title // NSString
           delegate:(id)delegate
  cancelButtonTitle:(id)cancel_title // NSString
destructiveButtonTitle:(id)destructive_title // NSString
  otherButtonTitles:(id)other_titles // NSString (first of variadic)
                ...:() {
    let this: id = msg_super![env; this init];
    if this == nil {
        return nil;
    }
    
    if title != nil {
        retain(env, title);
    }
    let host_obj = env.objc.borrow_mut::<UIActionSheetHostObject>(this);
    host_obj.title = title;
    host_obj.delegate = delegate;
    
    let mut button_index = 0;
    
    // Add destructive button if provided
    if destructive_title != nil {
        retain(env, destructive_title);
        host_obj.button_titles.push(destructive_title);
        host_obj.destructive_button_index = button_index;
        button_index += 1;
    }
    
    // Add other buttons if provided
    if other_titles != nil {
        retain(env, other_titles);
        host_obj.button_titles.push(other_titles);
        if host_obj.first_other_button_index == -1 {
            host_obj.first_other_button_index = button_index;
        }
        button_index += 1;
    }
    
    // Add cancel button if provided
    if cancel_title != nil {
        retain(env, cancel_title);
        host_obj.button_titles.push(cancel_title);
        host_obj.cancel_button_index = button_index;
    }
    
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return nil;
    }
    
    // Decode title
    let title_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITitle");
    let title: id = msg![env; coder decodeObjectForKey:title_key];
    if title != nil {
        retain(env, title);
        env.objc.borrow_mut::<UIActionSheetHostObject>(this).title = title;
    }
    
    // Decode action sheet style
    let style_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIActionSheetStyle");
    if msg![env; coder containsValueForKey:style_key] {
        let style: NSInteger = msg![env; coder decodeIntegerForKey:style_key];
        env.objc.borrow_mut::<UIActionSheetHostObject>(this).action_sheet_style = style;
    }
    
    // Decode cancel button index
    let cancel_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UICancelButtonIndex");
    if msg![env; coder containsValueForKey:cancel_key] {
        let index: NSInteger = msg![env; coder decodeIntegerForKey:cancel_key];
        env.objc.borrow_mut::<UIActionSheetHostObject>(this).cancel_button_index = index;
    }
    
    // Decode destructive button index
    let destructive_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIDestructiveButtonIndex");
    if msg![env; coder containsValueForKey:destructive_key] {
        let index: NSInteger = msg![env; coder decodeIntegerForKey:destructive_key];
        env.objc.borrow_mut::<UIActionSheetHostObject>(this).destructive_button_index = index;
    }
    
    this
}

- (())setTitle:(id)title { // NSString
    let host_obj = env.objc.borrow_mut::<UIActionSheetHostObject>(this);
    if host_obj.title != nil {
        release(env, host_obj.title);
    }
    if title != nil {
        retain(env, title);
    }
    host_obj.title = title;
}

- (id)title { // NSString
    env.objc.borrow::<UIActionSheetHostObject>(this).title
}

- (())setDelegate:(id)delegate {
    env.objc.borrow_mut::<UIActionSheetHostObject>(this).delegate = delegate;
}

- (id)delegate {
    env.objc.borrow::<UIActionSheetHostObject>(this).delegate
}

- (NSInteger)addButtonWithTitle:(id)title { // NSString
    if title == nil {
        return -1;
    }
    
    retain(env, title);
    let host_obj = env.objc.borrow_mut::<UIActionSheetHostObject>(this);
    host_obj.button_titles.push(title);
    (host_obj.button_titles.len() - 1) as NSInteger
}

- (NSInteger)numberOfButtons {
    env.objc.borrow::<UIActionSheetHostObject>(this).button_titles.len() as NSInteger
}

- (id)buttonTitleAtIndex:(NSInteger)index { // NSString
    let host_obj = env.objc.borrow::<UIActionSheetHostObject>(this);
    if index >= 0 && (index as usize) < host_obj.button_titles.len() {
        host_obj.button_titles[index as usize]
    } else {
        nil
    }
}

- (())setCancelButtonIndex:(NSInteger)index {
    env.objc.borrow_mut::<UIActionSheetHostObject>(this).cancel_button_index = index;
}

- (NSInteger)cancelButtonIndex {
    env.objc.borrow::<UIActionSheetHostObject>(this).cancel_button_index
}

- (())setDestructiveButtonIndex:(NSInteger)index {
    env.objc.borrow_mut::<UIActionSheetHostObject>(this).destructive_button_index = index;
}

- (NSInteger)destructiveButtonIndex {
    env.objc.borrow::<UIActionSheetHostObject>(this).destructive_button_index
}

- (NSInteger)firstOtherButtonIndex {
    env.objc.borrow::<UIActionSheetHostObject>(this).first_other_button_index
}

- (())setActionSheetStyle:(UIActionSheetStyle)style {
    env.objc.borrow_mut::<UIActionSheetHostObject>(this).action_sheet_style = style;
}

- (UIActionSheetStyle)actionSheetStyle {
    env.objc.borrow::<UIActionSheetHostObject>(this).action_sheet_style
}

- (bool)isVisible {
    env.objc.borrow::<UIActionSheetHostObject>(this).visible
}

- (())showInView:(id)view { // UIView
    log_dbg!("UIActionSheet {:?} showInView:{:?} (stub)", this, view);
    env.objc.borrow_mut::<UIActionSheetHostObject>(this).visible = true;
}

- (())dismissWithClickedButtonIndex:(NSInteger)button_index animated:(bool)animated {
    log_dbg!("UIActionSheet {:?} dismissed with button index {} (animated: {})", 
             this, button_index, animated);
    env.objc.borrow_mut::<UIActionSheetHostObject>(this).visible = false;
}

- (())dealloc {
    let host_obj = env.objc.borrow_mut::<UIActionSheetHostObject>(this);
    
    if host_obj.title != nil {
        release(env, host_obj.title);
    }
    
    for &button_title in &host_obj.button_titles {
        release(env, button_title);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
