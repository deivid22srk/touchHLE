/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UISearchBar`.

use super::ui_view::UIViewHostObject;
use crate::frameworks::core_graphics::CGRect;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

// UIBarStyle
type UIBarStyle = i32;
#[allow(dead_code)]
const UI_BAR_STYLE_DEFAULT: UIBarStyle = 0;
#[allow(dead_code)]
const UI_BAR_STYLE_BLACK: UIBarStyle = 1;

pub struct UISearchBarHostObject {
    superclass: UIViewHostObject,
    bar_style: UIBarStyle,
    text: id,           // NSString
    placeholder: id,    // NSString
    prompt: id,         // NSString
    delegate: id,       // weak reference
    tint_color: id,     // UIColor
    shows_cancel_button: bool,
    shows_search_results_button: bool,
    #[allow(dead_code)]
    translucent: bool,
}
impl_HostObject_with_superclass!(UISearchBarHostObject);

impl Default for UISearchBarHostObject {
    fn default() -> Self {
        UISearchBarHostObject {
            superclass: Default::default(),
            bar_style: UI_BAR_STYLE_DEFAULT,
            text: nil,
            placeholder: nil,
            prompt: nil,
            delegate: nil,
            tint_color: nil,
            shows_cancel_button: false,
            shows_search_results_button: false,
            translucent: false,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISearchBar: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UISearchBarHostObject>::default();
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
        env.objc.borrow_mut::<UISearchBarHostObject>(this).bar_style = bar_style;
    }
    
    // Decode text
    let text_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIText");
    let text: id = msg![env; coder decodeObjectForKey:text_key];
    if text != nil {
        retain(env, text);
        env.objc.borrow_mut::<UISearchBarHostObject>(this).text = text;
    }
    
    // Decode placeholder
    let placeholder_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIPlaceholder");
    let placeholder: id = msg![env; coder decodeObjectForKey:placeholder_key];
    if placeholder != nil {
        retain(env, placeholder);
        env.objc.borrow_mut::<UISearchBarHostObject>(this).placeholder = placeholder;
    }
    
    // Decode prompt
    let prompt_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIPrompt");
    let prompt: id = msg![env; coder decodeObjectForKey:prompt_key];
    if prompt != nil {
        retain(env, prompt);
        env.objc.borrow_mut::<UISearchBarHostObject>(this).prompt = prompt;
    }
    
    // Decode tint color
    let tint_color_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITintColor");
    let tint_color: id = msg![env; coder decodeObjectForKey:tint_color_key];
    if tint_color != nil {
        retain(env, tint_color);
        env.objc.borrow_mut::<UISearchBarHostObject>(this).tint_color = tint_color;
    }
    
    // Decode shows cancel button
    let shows_cancel_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIShowsCancelButton");
    if msg![env; coder containsValueForKey:shows_cancel_key] {
        let shows_cancel: bool = msg![env; coder decodeBoolForKey:shows_cancel_key];
        env.objc.borrow_mut::<UISearchBarHostObject>(this).shows_cancel_button = shows_cancel;
    }
    
    this
}

- (())setBarStyle:(UIBarStyle)style {
    env.objc.borrow_mut::<UISearchBarHostObject>(this).bar_style = style;
}

- (UIBarStyle)barStyle {
    env.objc.borrow::<UISearchBarHostObject>(this).bar_style
}

- (())setText:(id)text { // NSString
    let host_obj = env.objc.borrow_mut::<UISearchBarHostObject>(this);
    if host_obj.text != nil {
        release(env, host_obj.text);
    }
    if text != nil {
        retain(env, text);
    }
    host_obj.text = text;
}

- (id)text { // NSString
    env.objc.borrow::<UISearchBarHostObject>(this).text
}

- (())setPlaceholder:(id)placeholder { // NSString
    let host_obj = env.objc.borrow_mut::<UISearchBarHostObject>(this);
    if host_obj.placeholder != nil {
        release(env, host_obj.placeholder);
    }
    if placeholder != nil {
        retain(env, placeholder);
    }
    host_obj.placeholder = placeholder;
}

- (id)placeholder { // NSString
    env.objc.borrow::<UISearchBarHostObject>(this).placeholder
}

- (())setPrompt:(id)prompt { // NSString
    let host_obj = env.objc.borrow_mut::<UISearchBarHostObject>(this);
    if host_obj.prompt != nil {
        release(env, host_obj.prompt);
    }
    if prompt != nil {
        retain(env, prompt);
    }
    host_obj.prompt = prompt;
}

- (id)prompt { // NSString
    env.objc.borrow::<UISearchBarHostObject>(this).prompt
}

- (())setDelegate:(id)delegate {
    env.objc.borrow_mut::<UISearchBarHostObject>(this).delegate = delegate;
}

- (id)delegate {
    env.objc.borrow::<UISearchBarHostObject>(this).delegate
}

- (())setTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UISearchBarHostObject>(this);
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.tint_color = color;
}

- (id)tintColor { // UIColor
    env.objc.borrow::<UISearchBarHostObject>(this).tint_color
}

- (())setShowsCancelButton:(bool)shows {
    env.objc.borrow_mut::<UISearchBarHostObject>(this).shows_cancel_button = shows;
}

- (bool)showsCancelButton {
    env.objc.borrow::<UISearchBarHostObject>(this).shows_cancel_button
}

- (())setShowsCancelButton:(bool)shows animated:(bool)_animated {
    env.objc.borrow_mut::<UISearchBarHostObject>(this).shows_cancel_button = shows;
}

- (())setShowsSearchResultsButton:(bool)shows {
    env.objc.borrow_mut::<UISearchBarHostObject>(this).shows_search_results_button = shows;
}

- (bool)showsSearchResultsButton {
    env.objc.borrow::<UISearchBarHostObject>(this).shows_search_results_button
}

- (())dealloc {
    let host_obj = env.objc.borrow::<UISearchBarHostObject>(this);
    
    if host_obj.text != nil {
        release(env, host_obj.text);
    }
    if host_obj.placeholder != nil {
        release(env, host_obj.placeholder);
    }
    if host_obj.prompt != nil {
        release(env, host_obj.prompt);
    }
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
