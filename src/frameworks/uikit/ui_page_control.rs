/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIPageControl`.

use super::ui_view::ui_control::UIControlHostObject;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

pub struct UIPageControlHostObject {
    superclass: UIControlHostObject,
    number_of_pages: NSInteger,
    current_page: NSInteger,
    hides_for_single_page: bool,
    defersCurrentPageDisplay: bool,
    page_indicator_tint_color: id,     // UIColor
    current_page_indicator_tint_color: id,  // UIColor
}
impl_HostObject_with_superclass!(UIPageControlHostObject);

impl Default for UIPageControlHostObject {
    fn default() -> Self {
        UIPageControlHostObject {
            superclass: Default::default(),
            number_of_pages: 0,
            current_page: 0,
            hides_for_single_page: false,
            defersCurrentPageDisplay: false,
            page_indicator_tint_color: nil,
            current_page_indicator_tint_color: nil,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIPageControl: UIControl

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIPageControlHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return nil;
    }
    
    // Decode number of pages
    let pages_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UINumberOfPages");
    if msg![env; coder containsValueForKey:pages_key] {
        let pages: NSInteger = msg![env; coder decodeIntegerForKey:pages_key];
        env.objc.borrow_mut::<UIPageControlHostObject>(this).number_of_pages = pages;
    }
    
    // Decode current page
    let current_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UICurrentPage");
    if msg![env; coder containsValueForKey:current_key] {
        let current: NSInteger = msg![env; coder decodeIntegerForKey:current_key];
        env.objc.borrow_mut::<UIPageControlHostObject>(this).current_page = current;
    }
    
    // Decode hides for single page
    let hides_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIHidesForSinglePage");
    if msg![env; coder containsValueForKey:hides_key] {
        let hides: bool = msg![env; coder decodeBoolForKey:hides_key];
        env.objc.borrow_mut::<UIPageControlHostObject>(this).hides_for_single_page = hides;
    }
    
    // Decode defers current page display
    let defers_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIDefersCurrentPageDisplay");
    if msg![env; coder containsValueForKey:defers_key] {
        let defers: bool = msg![env; coder decodeBoolForKey:defers_key];
        env.objc.borrow_mut::<UIPageControlHostObject>(this).defersCurrentPageDisplay = defers;
    }
    
    // Decode tint colors
    let page_tint_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIPageIndicatorTintColor");
    let page_tint: id = msg![env; coder decodeObjectForKey:page_tint_key];
    if page_tint != nil {
        retain(env, page_tint);
        env.objc.borrow_mut::<UIPageControlHostObject>(this).page_indicator_tint_color = page_tint;
    }
    
    let current_tint_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UICurrentPageIndicatorTintColor");
    let current_tint: id = msg![env; coder decodeObjectForKey:current_tint_key];
    if current_tint != nil {
        retain(env, current_tint);
        env.objc.borrow_mut::<UIPageControlHostObject>(this).current_page_indicator_tint_color = current_tint;
    }
    
    this
}

- (())setNumberOfPages:(NSInteger)pages {
    let host_obj = env.objc.borrow_mut::<UIPageControlHostObject>(this);
    host_obj.number_of_pages = pages;
    
    // Adjust current page if out of bounds
    if host_obj.current_page >= pages {
        host_obj.current_page = pages.saturating_sub(1).max(0);
    }
}

- (NSInteger)numberOfPages {
    env.objc.borrow::<UIPageControlHostObject>(this).number_of_pages
}

- (())setCurrentPage:(NSInteger)page {
    let host_obj = env.objc.borrow_mut::<UIPageControlHostObject>(this);
    let clamped = page.clamp(0, host_obj.number_of_pages.saturating_sub(1).max(0));
    host_obj.current_page = clamped;
    log_dbg!("UIPageControl {:?} setCurrentPage: {}", this, clamped);
}

- (NSInteger)currentPage {
    env.objc.borrow::<UIPageControlHostObject>(this).current_page
}

- (())setHidesForSinglePage:(bool)hides {
    env.objc.borrow_mut::<UIPageControlHostObject>(this).hides_for_single_page = hides;
}

- (bool)hidesForSinglePage {
    env.objc.borrow::<UIPageControlHostObject>(this).hides_for_single_page
}

- (())setDefersCurrentPageDisplay:(bool)defers {
    env.objc.borrow_mut::<UIPageControlHostObject>(this).defersCurrentPageDisplay = defers;
}

- (bool)defersCurrentPageDisplay {
    env.objc.borrow::<UIPageControlHostObject>(this).defersCurrentPageDisplay
}

- (())updateCurrentPageDisplay {
    log_dbg!("UIPageControl {:?} updateCurrentPageDisplay", this);
}

- (())setPageIndicatorTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UIPageControlHostObject>(this);
    if host_obj.page_indicator_tint_color != nil {
        release(env, host_obj.page_indicator_tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.page_indicator_tint_color = color;
}

- (id)pageIndicatorTintColor { // UIColor
    env.objc.borrow::<UIPageControlHostObject>(this).page_indicator_tint_color
}

- (())setCurrentPageIndicatorTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UIPageControlHostObject>(this);
    if host_obj.current_page_indicator_tint_color != nil {
        release(env, host_obj.current_page_indicator_tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.current_page_indicator_tint_color = color;
}

- (id)currentPageIndicatorTintColor { // UIColor
    env.objc.borrow::<UIPageControlHostObject>(this).current_page_indicator_tint_color
}

- (())dealloc {
    let host_obj = env.objc.borrow::<UIPageControlHostObject>(this);
    
    if host_obj.page_indicator_tint_color != nil {
        release(env, host_obj.page_indicator_tint_color);
    }
    if host_obj.current_page_indicator_tint_color != nil {
        release(env, host_obj.current_page_indicator_tint_color);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
