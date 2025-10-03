/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIPageControl`.

use crate::frameworks::core_graphics::CGRect;
use crate::frameworks::foundation::ns_string::get_static_str;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

#[derive(Default)]
struct UIPageControlHostObject {
    superclass: super::ui_control::UIControlHostObject,
    number_of_pages: NSInteger,
    current_page: NSInteger,
    hides_for_single_page: bool,
    /// `UIColor*`
    page_indicator_tint_color: id,
    /// `UIColor*`
    current_page_indicator_tint_color: id,
}
impl_HostObject_with_superclass!(UIPageControlHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIPageControl: super::ui_control::UIControl

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIPageControlHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    env.objc.borrow_mut::<UIPageControlHostObject>(this).number_of_pages = 0;
    env.objc.borrow_mut::<UIPageControlHostObject>(this).current_page = 0;
    env.objc.borrow_mut::<UIPageControlHostObject>(this).hides_for_single_page = false;
    this
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    
    // Decode number of pages
    let num_pages_key = get_static_str(env, "UINumberOfPages");
    if msg![env; coder containsValueForKey:num_pages_key] {
        let num_pages: NSInteger = msg![env; coder decodeIntForKey:num_pages_key];
        () = msg![env; this setNumberOfPages:num_pages];
    }
    
    // Decode current page
    let current_page_key = get_static_str(env, "UICurrentPage");
    if msg![env; coder containsValueForKey:current_page_key] {
        let current_page: NSInteger = msg![env; coder decodeIntForKey:current_page_key];
        () = msg![env; this setCurrentPage:current_page];
    }
    
    // Decode hides for single page
    let hides_key = get_static_str(env, "UIHidesForSinglePage");
    if msg![env; coder containsValueForKey:hides_key] {
        let hides: bool = msg![env; coder decodeBoolForKey:hides_key];
        () = msg![env; this setHidesForSinglePage:hides];
    }
    
    // Decode page indicator tint color
    let page_indicator_tint_key = get_static_str(env, "UIPageIndicatorTintColor");
    let page_indicator_tint: id = msg![env; coder decodeObjectForKey:page_indicator_tint_key];
    if page_indicator_tint != nil {
        () = msg![env; this setPageIndicatorTintColor:page_indicator_tint];
    }
    
    // Decode current page indicator tint color
    let current_page_tint_key = get_static_str(env, "UICurrentPageIndicatorTintColor");
    let current_page_tint: id = msg![env; coder decodeObjectForKey:current_page_tint_key];
    if current_page_tint != nil {
        () = msg![env; this setCurrentPageIndicatorTintColor:current_page_tint];
    }
    
    this
}

- (())dealloc {
    let &UIPageControlHostObject {
        superclass: _,
        number_of_pages: _,
        current_page: _,
        hides_for_single_page: _,
        page_indicator_tint_color,
        current_page_indicator_tint_color,
    } = env.objc.borrow(this);
    release(env, page_indicator_tint_color);
    release(env, current_page_indicator_tint_color);
    msg_super![env; this dealloc]
}

- (NSInteger)numberOfPages {
    env.objc.borrow::<UIPageControlHostObject>(this).number_of_pages
}

- (())setNumberOfPages:(NSInteger)num_pages {
    env.objc.borrow_mut::<UIPageControlHostObject>(this).number_of_pages = num_pages;
    () = msg![env; this setNeedsDisplay];
}

- (NSInteger)currentPage {
    env.objc.borrow::<UIPageControlHostObject>(this).current_page
}

- (())setCurrentPage:(NSInteger)current_page {
    let num_pages: NSInteger = msg![env; this numberOfPages];
    let current_page = current_page.max(0).min(num_pages - 1);
    env.objc.borrow_mut::<UIPageControlHostObject>(this).current_page = current_page;
    () = msg![env; this setNeedsDisplay];
}

- (bool)hidesForSinglePage {
    env.objc.borrow::<UIPageControlHostObject>(this).hides_for_single_page
}

- (())setHidesForSinglePage:(bool)hides {
    env.objc.borrow_mut::<UIPageControlHostObject>(this).hides_for_single_page = hides;
}

- (id)pageIndicatorTintColor {
    env.objc.borrow::<UIPageControlHostObject>(this).page_indicator_tint_color
}

- (())setPageIndicatorTintColor:(id)color { // UIColor*
    let host_obj = env.objc.borrow_mut::<UIPageControlHostObject>(this);
    let old_color = std::mem::replace(&mut host_obj.page_indicator_tint_color, color);
    retain(env, color);
    release(env, old_color);
}

- (id)currentPageIndicatorTintColor {
    env.objc.borrow::<UIPageControlHostObject>(this).current_page_indicator_tint_color
}

- (())setCurrentPageIndicatorTintColor:(id)color { // UIColor*
    let host_obj = env.objc.borrow_mut::<UIPageControlHostObject>(this);
    let old_color = std::mem::replace(&mut host_obj.current_page_indicator_tint_color, color);
    retain(env, color);
    release(env, old_color);
}

@end

};
