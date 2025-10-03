/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSLayoutConstraint` for Auto Layout support in NIBs.

use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, msg, msg_super, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr,
};

// NSLayoutRelation
type NSLayoutRelation = NSInteger;
#[allow(dead_code)]
const NS_LAYOUT_RELATION_LESS_THAN_OR_EQUAL: NSLayoutRelation = -1;
#[allow(dead_code)]
const NS_LAYOUT_RELATION_EQUAL: NSLayoutRelation = 0;
#[allow(dead_code)]
const NS_LAYOUT_RELATION_GREATER_THAN_OR_EQUAL: NSLayoutRelation = 1;

// NSLayoutAttribute
type NSLayoutAttribute = NSInteger;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_LEFT: NSLayoutAttribute = 1;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_RIGHT: NSLayoutAttribute = 2;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_TOP: NSLayoutAttribute = 3;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_BOTTOM: NSLayoutAttribute = 4;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_LEADING: NSLayoutAttribute = 5;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_TRAILING: NSLayoutAttribute = 6;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_WIDTH: NSLayoutAttribute = 7;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_HEIGHT: NSLayoutAttribute = 8;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_CENTER_X: NSLayoutAttribute = 9;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_CENTER_Y: NSLayoutAttribute = 10;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_BASELINE: NSLayoutAttribute = 11;
#[allow(dead_code)]
const NS_LAYOUT_ATTRIBUTE_NOT_AN_ATTRIBUTE: NSLayoutAttribute = 0;

// NSLayoutPriority
type NSLayoutPriority = f32;
#[allow(dead_code)]
const UI_LAYOUT_PRIORITY_REQUIRED: NSLayoutPriority = 1000.0;
#[allow(dead_code)]
const UI_LAYOUT_PRIORITY_DEFAULT_HIGH: NSLayoutPriority = 750.0;
#[allow(dead_code)]
const UI_LAYOUT_PRIORITY_DEFAULT_LOW: NSLayoutPriority = 250.0;

pub struct NSLayoutConstraintHostObject {
    first_item: id,         // UIView (weak)
    first_attribute: NSLayoutAttribute,
    relation: NSLayoutRelation,
    second_item: id,        // UIView (weak)
    second_attribute: NSLayoutAttribute,
    multiplier: f32,
    constant: f32,
    priority: NSLayoutPriority,
    #[allow(dead_code)]
    should_be_archived: bool,
    active: bool,
}
impl HostObject for NSLayoutConstraintHostObject {}

impl Default for NSLayoutConstraintHostObject {
    fn default() -> Self {
        NSLayoutConstraintHostObject {
            first_item: nil,
            first_attribute: NS_LAYOUT_ATTRIBUTE_NOT_AN_ATTRIBUTE,
            relation: NS_LAYOUT_RELATION_EQUAL,
            second_item: nil,
            second_attribute: NS_LAYOUT_ATTRIBUTE_NOT_AN_ATTRIBUTE,
            multiplier: 1.0,
            constant: 0.0,
            priority: UI_LAYOUT_PRIORITY_REQUIRED,
            should_be_archived: true,
            active: false,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSLayoutConstraint: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<NSLayoutConstraintHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this init];
    if this == nil {
        return nil;
    }
    
    // Decode first item (weak reference)
    let first_item_key = crate::frameworks::foundation::ns_string::get_static_str(env, "NSFirstItem");
    let first_item: id = msg![env; coder decodeObjectForKey:first_item_key];
    env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).first_item = first_item;
    
    // Decode first attribute
    let first_attr_key = crate::frameworks::foundation::ns_string::get_static_str(env, "NSFirstAttribute");
    if msg![env; coder containsValueForKey:first_attr_key] {
        let attr: NSInteger = msg![env; coder decodeIntegerForKey:first_attr_key];
        env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).first_attribute = attr;
    }
    
    // Decode relation
    let relation_key = crate::frameworks::foundation::ns_string::get_static_str(env, "NSRelation");
    if msg![env; coder containsValueForKey:relation_key] {
        let relation: NSInteger = msg![env; coder decodeIntegerForKey:relation_key];
        env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).relation = relation;
    }
    
    // Decode second item (weak reference)
    let second_item_key = crate::frameworks::foundation::ns_string::get_static_str(env, "NSSecondItem");
    let second_item: id = msg![env; coder decodeObjectForKey:second_item_key];
    env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).second_item = second_item;
    
    // Decode second attribute
    let second_attr_key = crate::frameworks::foundation::ns_string::get_static_str(env, "NSSecondAttribute");
    if msg![env; coder containsValueForKey:second_attr_key] {
        let attr: NSInteger = msg![env; coder decodeIntegerForKey:second_attr_key];
        env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).second_attribute = attr;
    }
    
    // Decode multiplier
    let multiplier_key = crate::frameworks::foundation::ns_string::get_static_str(env, "NSMultiplier");
    if msg![env; coder containsValueForKey:multiplier_key] {
        let multiplier: f32 = msg![env; coder decodeFloatForKey:multiplier_key];
        env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).multiplier = multiplier;
    }
    
    // Decode constant
    let constant_key = crate::frameworks::foundation::ns_string::get_static_str(env, "NSConstant");
    if msg![env; coder containsValueForKey:constant_key] {
        let constant: f32 = msg![env; coder decodeFloatForKey:constant_key];
        env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).constant = constant;
    }
    
    // Decode priority
    let priority_key = crate::frameworks::foundation::ns_string::get_static_str(env, "NSPriority");
    if msg![env; coder containsValueForKey:priority_key] {
        let priority: f32 = msg![env; coder decodeFloatForKey:priority_key];
        env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).priority = priority;
    }
    
    log_dbg!("NSLayoutConstraint loaded from NIB (stub implementation - constraints not applied)");
    
    this
}

- (id)firstItem {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).first_item
}

- (NSLayoutAttribute)firstAttribute {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).first_attribute
}

- (NSLayoutRelation)relation {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).relation
}

- (id)secondItem {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).second_item
}

- (NSLayoutAttribute)secondAttribute {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).second_attribute
}

- (f32)multiplier {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).multiplier
}

- (())setConstant:(f32)constant {
    env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).constant = constant;
}

- (f32)constant {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).constant
}

- (())setPriority:(NSLayoutPriority)priority {
    env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).priority = priority;
}

- (NSLayoutPriority)priority {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).priority
}

- (())setActive:(bool)active {
    env.objc.borrow_mut::<NSLayoutConstraintHostObject>(this).active = active;
}

- (bool)isActive {
    env.objc.borrow::<NSLayoutConstraintHostObject>(this).active
}

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};
