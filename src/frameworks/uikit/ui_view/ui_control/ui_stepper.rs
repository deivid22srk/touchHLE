/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIStepper`.

use super::UIControlHostObject;
use crate::frameworks::core_graphics::CGRect;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

pub struct UIStepperHostObject {
    superclass: UIControlHostObject,
    value: f64,
    minimum_value: f64,
    maximum_value: f64,
    step_value: f64,
    continuous: bool,
    autorepeat: bool,
    wraps: bool,
    tint_color: id, // UIColor
}
impl_HostObject_with_superclass!(UIStepperHostObject);

impl Default for UIStepperHostObject {
    fn default() -> Self {
        UIStepperHostObject {
            superclass: Default::default(),
            value: 0.0,
            minimum_value: 0.0,
            maximum_value: 100.0,
            step_value: 1.0,
            continuous: true,
            autorepeat: true,
            wraps: false,
            tint_color: nil,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIStepper: UIControl

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIStepperHostObject>::default();
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
    
    // Decode value
    let value_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIValue");
    if msg![env; coder containsValueForKey:value_key] {
        let value: f64 = msg![env; coder decodeDoubleForKey:value_key];
        env.objc.borrow_mut::<UIStepperHostObject>(this).value = value;
    }
    
    // Decode minimum value
    let min_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIMinimumValue");
    if msg![env; coder containsValueForKey:min_key] {
        let min: f64 = msg![env; coder decodeDoubleForKey:min_key];
        env.objc.borrow_mut::<UIStepperHostObject>(this).minimum_value = min;
    }
    
    // Decode maximum value
    let max_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIMaximumValue");
    if msg![env; coder containsValueForKey:max_key] {
        let max: f64 = msg![env; coder decodeDoubleForKey:max_key];
        env.objc.borrow_mut::<UIStepperHostObject>(this).maximum_value = max;
    }
    
    // Decode step value
    let step_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIStepValue");
    if msg![env; coder containsValueForKey:step_key] {
        let step: f64 = msg![env; coder decodeDoubleForKey:step_key];
        env.objc.borrow_mut::<UIStepperHostObject>(this).step_value = step;
    }
    
    // Decode continuous
    let continuous_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIContinuous");
    if msg![env; coder containsValueForKey:continuous_key] {
        let continuous: bool = msg![env; coder decodeBoolForKey:continuous_key];
        env.objc.borrow_mut::<UIStepperHostObject>(this).continuous = continuous;
    }
    
    // Decode autorepeat
    let autorepeat_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIAutorepeat");
    if msg![env; coder containsValueForKey:autorepeat_key] {
        let autorepeat: bool = msg![env; coder decodeBoolForKey:autorepeat_key];
        env.objc.borrow_mut::<UIStepperHostObject>(this).autorepeat = autorepeat;
    }
    
    // Decode wraps
    let wraps_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIWraps");
    if msg![env; coder containsValueForKey:wraps_key] {
        let wraps: bool = msg![env; coder decodeBoolForKey:wraps_key];
        env.objc.borrow_mut::<UIStepperHostObject>(this).wraps = wraps;
    }
    
    // Decode tint color
    let tint_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UITintColor");
    let tint: id = msg![env; coder decodeObjectForKey:tint_key];
    if tint != nil {
        retain(env, tint);
        env.objc.borrow_mut::<UIStepperHostObject>(this).tint_color = tint;
    }
    
    this
}

- (())setValue:(f64)value {
    let host_obj = env.objc.borrow_mut::<UIStepperHostObject>(this);
    let clamped = value.clamp(host_obj.minimum_value, host_obj.maximum_value);
    host_obj.value = clamped;
}

- (f64)value {
    env.objc.borrow::<UIStepperHostObject>(this).value
}

- (())setMinimumValue:(f64)min {
    env.objc.borrow_mut::<UIStepperHostObject>(this).minimum_value = min;
}

- (f64)minimumValue {
    env.objc.borrow::<UIStepperHostObject>(this).minimum_value
}

- (())setMaximumValue:(f64)max {
    env.objc.borrow_mut::<UIStepperHostObject>(this).maximum_value = max;
}

- (f64)maximumValue {
    env.objc.borrow::<UIStepperHostObject>(this).maximum_value
}

- (())setStepValue:(f64)step {
    env.objc.borrow_mut::<UIStepperHostObject>(this).step_value = step;
}

- (f64)stepValue {
    env.objc.borrow::<UIStepperHostObject>(this).step_value
}

- (())setContinuous:(bool)continuous {
    env.objc.borrow_mut::<UIStepperHostObject>(this).continuous = continuous;
}

- (bool)isContinuous {
    env.objc.borrow::<UIStepperHostObject>(this).continuous
}

- (())setAutorepeat:(bool)autorepeat {
    env.objc.borrow_mut::<UIStepperHostObject>(this).autorepeat = autorepeat;
}

- (bool)autorepeat {
    env.objc.borrow::<UIStepperHostObject>(this).autorepeat
}

- (())setWraps:(bool)wraps {
    env.objc.borrow_mut::<UIStepperHostObject>(this).wraps = wraps;
}

- (bool)wraps {
    env.objc.borrow::<UIStepperHostObject>(this).wraps
}

- (())setTintColor:(id)color { // UIColor
    let host_obj = env.objc.borrow_mut::<UIStepperHostObject>(this);
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    if color != nil {
        retain(env, color);
    }
    host_obj.tint_color = color;
}

- (id)tintColor { // UIColor
    env.objc.borrow::<UIStepperHostObject>(this).tint_color
}

- (())dealloc {
    let host_obj = env.objc.borrow::<UIStepperHostObject>(this);
    
    if host_obj.tint_color != nil {
        release(env, host_obj.tint_color);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
