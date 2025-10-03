/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITabBarController`.

use super::UIViewControllerHostObject;
use crate::frameworks::foundation::NSUInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

pub struct UITabBarControllerHostObject {
    superclass: UIViewControllerHostObject,
    view_controllers: Vec<id>,  // NSArray of UIViewController
    selected_view_controller: id,
    selected_index: NSUInteger,
    tab_bar: id,                // UITabBar
    delegate: id,               // weak reference
    more_navigation_controller: id,
    #[allow(dead_code)]
    customizable_view_controllers: Vec<id>,
}
impl_HostObject_with_superclass!(UITabBarControllerHostObject);

impl Default for UITabBarControllerHostObject {
    fn default() -> Self {
        UITabBarControllerHostObject {
            superclass: Default::default(),
            view_controllers: Vec::new(),
            selected_view_controller: nil,
            selected_index: 0,
            tab_bar: nil,
            delegate: nil,
            more_navigation_controller: nil,
            customizable_view_controllers: Vec::new(),
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITabBarController: UIViewController

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITabBarControllerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)init {
    let this: id = msg_super![env; this init];
    if this != nil {
        // Create tab bar
        let tab_bar_class = env.objc.get_known_class("UITabBar", &mut env.mem);
        let tab_bar: id = msg![env; tab_bar_class alloc];
        let frame = crate::frameworks::core_graphics::CGRect {
            origin: crate::frameworks::core_graphics::CGPoint { x: 0.0, y: 0.0 },
            size: crate::frameworks::core_graphics::CGSize { width: 320.0, height: 49.0 },
        };
        let tab_bar: id = msg![env; tab_bar initWithFrame:frame];
        retain(env, tab_bar);
        env.objc.borrow_mut::<UITabBarControllerHostObject>(this).tab_bar = tab_bar;
        
        log_dbg!("UITabBarController {:?} initialized with tab bar {:?}", this, tab_bar);
    }
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return nil;
    }
    
    // Decode view controllers
    let vcs_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIViewControllers");
    let vcs: id = msg![env; coder decodeObjectForKey:vcs_key];
    if vcs != nil {
        let count: NSUInteger = msg![env; vcs count];
        let host_obj = env.objc.borrow_mut::<UITabBarControllerHostObject>(this);
        for i in 0..count {
            let vc: id = msg![env; vcs objectAtIndex:i];
            retain(env, vc);
            host_obj.view_controllers.push(vc);
        }
    }
    
    // Decode selected index
    let selected_index_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UISelectedIndex");
    if msg![env; coder containsValueForKey:selected_index_key] {
        let index: i32 = msg![env; coder decodeIntForKey:selected_index_key];
        env.objc.borrow_mut::<UITabBarControllerHostObject>(this).selected_index = index as NSUInteger;
    }
    
    // Decode tab bar
    let tab_bar_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITabBar");
    let tab_bar: id = msg![env; coder decodeObjectForKey:tab_bar_key];
    if tab_bar != nil {
        retain(env, tab_bar);
        env.objc.borrow_mut::<UITabBarControllerHostObject>(this).tab_bar = tab_bar;
    }
    
    this
}

- (())setViewControllers:(id)view_controllers animated:(bool)animated { // NSArray
    let host_obj = env.objc.borrow_mut::<UITabBarControllerHostObject>(this);
    
    // Release old view controllers
    for &vc in &host_obj.view_controllers {
        release(env, vc);
    }
    host_obj.view_controllers.clear();
    
    if view_controllers != nil {
        let count: NSUInteger = msg![env; view_controllers count];
        for i in 0..count {
            let vc: id = msg![env; view_controllers objectAtIndex:i];
            retain(env, vc);
            host_obj.view_controllers.push(vc);
        }
        log_dbg!("UITabBarController: set {} view controllers (animated: {})", count, animated);
    }
}

- (())setViewControllers:(id)view_controllers { // NSArray
    () = msg![env; this setViewControllers:view_controllers animated:false]
}

- (id)viewControllers { // NSArray
    let host_obj = env.objc.borrow::<UITabBarControllerHostObject>(this);
    crate::frameworks::foundation::ns_array::from_vec(env, host_obj.view_controllers.clone())
}

- (())setSelectedViewController:(id)vc { // UIViewController
    let host_obj = env.objc.borrow_mut::<UITabBarControllerHostObject>(this);
    if host_obj.selected_view_controller != nil {
        release(env, host_obj.selected_view_controller);
    }
    if vc != nil {
        retain(env, vc);
    }
    host_obj.selected_view_controller = vc;
}

- (id)selectedViewController { // UIViewController
    env.objc.borrow::<UITabBarControllerHostObject>(this).selected_view_controller
}

- (())setSelectedIndex:(NSUInteger)index {
    let host_obj = env.objc.borrow_mut::<UITabBarControllerHostObject>(this);
    host_obj.selected_index = index;
    
    // Update selected view controller
    if (index as usize) < host_obj.view_controllers.len() {
        let vc = host_obj.view_controllers[index as usize];
        if host_obj.selected_view_controller != vc {
            if host_obj.selected_view_controller != nil {
                release(env, host_obj.selected_view_controller);
            }
            retain(env, vc);
            host_obj.selected_view_controller = vc;
        }
    }
}

- (NSUInteger)selectedIndex {
    env.objc.borrow::<UITabBarControllerHostObject>(this).selected_index
}

- (())setDelegate:(id)delegate {
    env.objc.borrow_mut::<UITabBarControllerHostObject>(this).delegate = delegate;
}

- (id)delegate {
    env.objc.borrow::<UITabBarControllerHostObject>(this).delegate
}

- (id)tabBar { // UITabBar
    let host_obj = env.objc.borrow::<UITabBarControllerHostObject>(this);
    host_obj.tab_bar
}

- (id)moreNavigationController { // UINavigationController
    env.objc.borrow::<UITabBarControllerHostObject>(this).more_navigation_controller
}

- (())dealloc {
    let host_obj = env.objc.borrow_mut::<UITabBarControllerHostObject>(this);
    
    for &vc in &host_obj.view_controllers {
        release(env, vc);
    }
    
    if host_obj.selected_view_controller != nil {
        release(env, host_obj.selected_view_controller);
    }
    
    if host_obj.tab_bar != nil {
        release(env, host_obj.tab_bar);
    }
    
    if host_obj.more_navigation_controller != nil {
        release(env, host_obj.more_navigation_controller);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
