/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITabBar` and `UITabBarItem`.

use super::ui_view::UIViewHostObject;
use crate::frameworks::core_graphics::CGRect;
use crate::frameworks::foundation::{ns_array, NSInteger, NSUInteger};
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, HostObject, NSZonePtr,
};

// UITabBarSystemItem
type UITabBarSystemItem = NSInteger;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_MORE: UITabBarSystemItem = 0;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_FAVORITES: UITabBarSystemItem = 1;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_FEATURED: UITabBarSystemItem = 2;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_TOP_RATED: UITabBarSystemItem = 3;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_RECENTS: UITabBarSystemItem = 4;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_CONTACTS: UITabBarSystemItem = 5;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_HISTORY: UITabBarSystemItem = 6;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_BOOKMARKS: UITabBarSystemItem = 7;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_SEARCH: UITabBarSystemItem = 8;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_DOWNLOADS: UITabBarSystemItem = 9;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_MOST_RECENT: UITabBarSystemItem = 10;
#[allow(dead_code)]
const UI_TAB_BAR_SYSTEM_ITEM_MOST_VIEWED: UITabBarSystemItem = 11;

pub struct UITabBarHostObject {
    superclass: UIViewHostObject,
    items: Vec<id>,         // NSArray of UITabBarItem
    selected_item: id,      // UITabBarItem
    delegate: id,           // weak reference
    tint_color: id,         // UIColor
}
impl_HostObject_with_superclass!(UITabBarHostObject);

impl Default for UITabBarHostObject {
    fn default() -> Self {
        UITabBarHostObject {
            superclass: Default::default(),
            items: Vec::new(),
            selected_item: nil,
            delegate: nil,
            tint_color: nil,
        }
    }
}

pub struct UITabBarItemHostObject {
    title: id,              // NSString
    image: id,              // UIImage
    badge_value: id,        // NSString
    tag: NSInteger,
    enabled: bool,
    #[allow(dead_code)]
    system_item: Option<UITabBarSystemItem>,
}
impl HostObject for UITabBarItemHostObject {}

impl Default for UITabBarItemHostObject {
    fn default() -> Self {
        UITabBarItemHostObject {
            title: nil,
            image: nil,
            badge_value: nil,
            tag: 0,
            enabled: true,
            system_item: None,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITabBar: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITabBarHostObject>::default();
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
    
    // Decode items
    let items_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIItems");
    let items: id = msg![env; coder decodeObjectForKey:items_key];
    if items != nil {
        let count: NSUInteger = msg![env; items count];
        let host_obj = env.objc.borrow_mut::<UITabBarHostObject>(this);
        for i in 0..count {
            let item: id = msg![env; items objectAtIndex:i];
            retain(env, item);
            host_obj.items.push(item);
        }
    }
    
    // Decode selected item
    let selected_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UISelectedItem");
    let selected: id = msg![env; coder decodeObjectForKey:selected_key];
    if selected != nil {
        retain(env, selected);
        env.objc.borrow_mut::<UITabBarHostObject>(this).selected_item = selected;
    }
    
    // Decode tint color
    let tint_color_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITintColor");
    let tint_color: id = msg![env; coder decodeObjectForKey:tint_color_key];
    if tint_color != nil {
        retain(env, tint_color);
        env.objc.borrow_mut::<UITabBarHostObject>(this).tint_color = tint_color;
    }
    
    this
}

- (())setDelegate:(id)delegate {
    env.objc.borrow_mut::<UITabBarHostObject>(this).delegate = delegate;
}

- (id)delegate {
    env.objc.borrow::<UITabBarHostObject>(this).delegate
}

- (())setItems:(id)items animated:(bool)animated { // NSArray of UITabBarItem
    let host_obj = env.objc.borrow_mut::<UITabBarHostObject>(this);
    
    // Release old items
    for &item in &host_obj.items {
        release(env, item);
    }
    host_obj.items.clear();
    
    if items != nil {
        let count: NSUInteger = msg![env; items count];
        for i in 0..count {
            let item: id = msg![env; items objectAtIndex:i];
            retain(env, item);
            host_obj.items.push(item);
        }
    }
    
    log_dbg!("UITabBar: set {} items (animated: {})", host_obj.items.len(), animated);
}

- (())setItems:(id)items { // NSArray
    () = msg![env; this setItems:items animated:false]
}

- (id)items { // NSArray
    let host_obj = env.objc.borrow::<UITabBarHostObject>(this);
    ns_array::from_vec(env, host_obj.items.clone())
}

- (())setSelectedItem:(id)item { // UITabBarItem
    let host_obj = env.objc.borrow_mut::<UITabBarHostObject>(this);
    if host_obj.selected_item != nil {
        release(env, host_obj.selected_item);
    }
    if item != nil {
        retain(env, item);
    }
    host_obj.selected_item = item;
}

- (id)selectedItem { // UITabBarItem
    env.objc.borrow::<UITabBarHostObject>(this).selected_item
}

- (())setTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UITabBarHostObject>(this);
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.tint_color = color;
}

- (id)tintColor { // UIColor
    env.objc.borrow::<UITabBarHostObject>(this).tint_color
}

- (())dealloc {
    let host_obj = env.objc.borrow_mut::<UITabBarHostObject>(this);
    
    for &item in &host_obj.items {
        release(env, item);
    }
    
    if host_obj.selected_item != nil {
        release(env, host_obj.selected_item);
    }
    
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    
    () = msg_super![env; this dealloc]
}

@end

@implementation UITabBarItem: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITabBarItemHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithTitle:(id)title // NSString
              image:(id)image // UIImage
                tag:(NSInteger)tag {
    let this: id = msg_super![env; this init];
    if this != nil {
        if title != nil {
            retain(env, title);
        }
        if image != nil {
            retain(env, image);
        }
        let host_obj = env.objc.borrow_mut::<UITabBarItemHostObject>(this);
        host_obj.title = title;
        host_obj.image = image;
        host_obj.tag = tag;
    }
    this
}

- (id)initWithTabBarSystemItem:(UITabBarSystemItem)system_item
                           tag:(NSInteger)tag {
    let this: id = msg_super![env; this init];
    if this != nil {
        let host_obj = env.objc.borrow_mut::<UITabBarItemHostObject>(this);
        host_obj.system_item = Some(system_item);
        host_obj.tag = tag;
        
        // Set default title based on system item
        use crate::frameworks::foundation::ns_string::from_rust_string;
        let title_str = match system_item {
            UI_TAB_BAR_SYSTEM_ITEM_MORE => "More",
            UI_TAB_BAR_SYSTEM_ITEM_FAVORITES => "Favorites",
            UI_TAB_BAR_SYSTEM_ITEM_FEATURED => "Featured",
            UI_TAB_BAR_SYSTEM_ITEM_TOP_RATED => "Top Rated",
            UI_TAB_BAR_SYSTEM_ITEM_RECENTS => "Recents",
            UI_TAB_BAR_SYSTEM_ITEM_CONTACTS => "Contacts",
            UI_TAB_BAR_SYSTEM_ITEM_HISTORY => "History",
            UI_TAB_BAR_SYSTEM_ITEM_BOOKMARKS => "Bookmarks",
            UI_TAB_BAR_SYSTEM_ITEM_SEARCH => "Search",
            UI_TAB_BAR_SYSTEM_ITEM_DOWNLOADS => "Downloads",
            UI_TAB_BAR_SYSTEM_ITEM_MOST_RECENT => "Most Recent",
            UI_TAB_BAR_SYSTEM_ITEM_MOST_VIEWED => "Most Viewed",
            _ => "",
        };
        if !title_str.is_empty() {
            let title = from_rust_string(env, title_str.to_string());
            host_obj.title = title;
        }
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
        env.objc.borrow_mut::<UITabBarItemHostObject>(this).title = title;
    }
    
    // Decode image
    let image_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIImage");
    let image: id = msg![env; coder decodeObjectForKey:image_key];
    if image != nil {
        retain(env, image);
        env.objc.borrow_mut::<UITabBarItemHostObject>(this).image = image;
    }
    
    // Decode badge value
    let badge_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIBadgeValue");
    let badge: id = msg![env; coder decodeObjectForKey:badge_key];
    if badge != nil {
        retain(env, badge);
        env.objc.borrow_mut::<UITabBarItemHostObject>(this).badge_value = badge;
    }
    
    // Decode tag
    let tag_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITag");
    if msg![env; coder containsValueForKey:tag_key] {
        let tag: NSInteger = msg![env; coder decodeIntegerForKey:tag_key];
        env.objc.borrow_mut::<UITabBarItemHostObject>(this).tag = tag;
    }
    
    // Decode enabled
    let enabled_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIEnabled");
    if msg![env; coder containsValueForKey:enabled_key] {
        let enabled: bool = msg![env; coder decodeBoolForKey:enabled_key];
        env.objc.borrow_mut::<UITabBarItemHostObject>(this).enabled = enabled;
    }
    
    this
}

- (())setTitle:(id)title { // NSString
    let host_obj = env.objc.borrow_mut::<UITabBarItemHostObject>(this);
    if host_obj.title != nil {
        release(env, host_obj.title);
    }
    if title != nil {
        retain(env, title);
    }
    host_obj.title = title;
}

- (id)title { // NSString
    env.objc.borrow::<UITabBarItemHostObject>(this).title
}

- (())setImage:(id)image { // UIImage
    let host_obj = env.objc.borrow_mut::<UITabBarItemHostObject>(this);
    if host_obj.image != nil {
        release(env, host_obj.image);
    }
    if image != nil {
        retain(env, image);
    }
    host_obj.image = image;
}

- (id)image { // UIImage
    env.objc.borrow::<UITabBarItemHostObject>(this).image
}

- (())setBadgeValue:(id)badge { // NSString
    let host_obj = env.objc.borrow_mut::<UITabBarItemHostObject>(this);
    if host_obj.badge_value != nil {
        release(env, host_obj.badge_value);
    }
    if badge != nil {
        retain(env, badge);
    }
    host_obj.badge_value = badge;
}

- (id)badgeValue { // NSString
    env.objc.borrow::<UITabBarItemHostObject>(this).badge_value
}

- (())setTag:(NSInteger)tag {
    env.objc.borrow_mut::<UITabBarItemHostObject>(this).tag = tag;
}

- (NSInteger)tag {
    env.objc.borrow::<UITabBarItemHostObject>(this).tag
}

- (())setEnabled:(bool)enabled {
    env.objc.borrow_mut::<UITabBarItemHostObject>(this).enabled = enabled;
}

- (bool)isEnabled {
    env.objc.borrow::<UITabBarItemHostObject>(this).enabled
}

- (())dealloc {
    let host_obj = env.objc.borrow::<UITabBarItemHostObject>(this);
    
    if host_obj.title != nil {
        release(env, host_obj.title);
    }
    if host_obj.image != nil {
        release(env, host_obj.image);
    }
    if host_obj.badge_value != nil {
        release(env, host_obj.badge_value);
    }
    
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
