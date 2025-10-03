/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIGestureRecognizer` and common gesture recognizers.

use crate::frameworks::core_graphics::CGPoint;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, msg, msg_super, nil, objc_classes, release, ClassExports, HostObject, NSZonePtr, SEL,
};

// UIGestureRecognizerState
type UIGestureRecognizerState = NSInteger;
#[allow(dead_code)]
const UI_GESTURE_RECOGNIZER_STATE_POSSIBLE: UIGestureRecognizerState = 0;
#[allow(dead_code)]
const UI_GESTURE_RECOGNIZER_STATE_BEGAN: UIGestureRecognizerState = 1;
#[allow(dead_code)]
const UI_GESTURE_RECOGNIZER_STATE_CHANGED: UIGestureRecognizerState = 2;
#[allow(dead_code)]
const UI_GESTURE_RECOGNIZER_STATE_ENDED: UIGestureRecognizerState = 3;
#[allow(dead_code)]
const UI_GESTURE_RECOGNIZER_STATE_CANCELLED: UIGestureRecognizerState = 4;
#[allow(dead_code)]
const UI_GESTURE_RECOGNIZER_STATE_FAILED: UIGestureRecognizerState = 5;

pub struct UIGestureRecognizerHostObject {
    target: id,         // weak reference
    action: Option<SEL>,
    view: id,           // UIView (weak)
    state: UIGestureRecognizerState,
    enabled: bool,
    cancels_touches_in_view: bool,
    delays_touches_began: bool,
    delays_touches_ended: bool,
}
impl HostObject for UIGestureRecognizerHostObject {}

impl Default for UIGestureRecognizerHostObject {
    fn default() -> Self {
        UIGestureRecognizerHostObject {
            target: nil,
            action: None,
            view: nil,
            state: UI_GESTURE_RECOGNIZER_STATE_POSSIBLE,
            enabled: true,
            cancels_touches_in_view: true,
            delays_touches_began: false,
            delays_touches_ended: true,
        }
    }
}

pub struct UITapGestureRecognizerHostObject {
    gesture_recognizer: UIGestureRecognizerHostObject,
    number_of_taps_required: NSInteger,
    number_of_touches_required: NSInteger,
}
impl HostObject for UITapGestureRecognizerHostObject {}

impl Default for UITapGestureRecognizerHostObject {
    fn default() -> Self {
        UITapGestureRecognizerHostObject {
            gesture_recognizer: Default::default(),
            number_of_taps_required: 1,
            number_of_touches_required: 1,
        }
    }
}

pub struct UISwipeGestureRecognizerHostObject {
    gesture_recognizer: UIGestureRecognizerHostObject,
    direction: NSInteger, // UISwipeGestureRecognizerDirection
    number_of_touches_required: NSInteger,
}
impl HostObject for UISwipeGestureRecognizerHostObject {}

impl Default for UISwipeGestureRecognizerHostObject {
    fn default() -> Self {
        UISwipeGestureRecognizerHostObject {
            gesture_recognizer: Default::default(),
            direction: 1, // UISwipeGestureRecognizerDirectionRight
            number_of_touches_required: 1,
        }
    }
}

pub struct UIPinchGestureRecognizerHostObject {
    gesture_recognizer: UIGestureRecognizerHostObject,
    scale: f32,
    velocity: f32,
}
impl HostObject for UIPinchGestureRecognizerHostObject {}

impl Default for UIPinchGestureRecognizerHostObject {
    fn default() -> Self {
        UIPinchGestureRecognizerHostObject {
            gesture_recognizer: Default::default(),
            scale: 1.0,
            velocity: 0.0,
        }
    }
}

pub struct UIPanGestureRecognizerHostObject {
    gesture_recognizer: UIGestureRecognizerHostObject,
    translation: CGPoint,
    velocity: CGPoint,
    minimum_number_of_touches: NSInteger,
    maximum_number_of_touches: NSInteger,
}
impl HostObject for UIPanGestureRecognizerHostObject {}

impl Default for UIPanGestureRecognizerHostObject {
    fn default() -> Self {
        UIPanGestureRecognizerHostObject {
            gesture_recognizer: Default::default(),
            translation: CGPoint { x: 0.0, y: 0.0 },
            velocity: CGPoint { x: 0.0, y: 0.0 },
            minimum_number_of_touches: 1,
            maximum_number_of_touches: std::i32::MAX,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIGestureRecognizer: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIGestureRecognizerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithTarget:(id)target action:(SEL)action {
    let this: id = msg_super![env; this init];
    if this != nil {
        let host_obj = env.objc.borrow_mut::<UIGestureRecognizerHostObject>(this);
        host_obj.target = target;
        host_obj.action = if action.is_null() { None } else { Some(action) };
    }
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this init];
    if this == nil {
        return nil;
    }
    
    // Decode enabled
    let enabled_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIEnabled");
    if msg![env; coder containsValueForKey:enabled_key] {
        let enabled: bool = msg![env; coder decodeBoolForKey:enabled_key];
        env.objc.borrow_mut::<UIGestureRecognizerHostObject>(this).enabled = enabled;
    }
    
    // Decode cancels touches in view
    let cancels_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UICancelsTouchesInView");
    if msg![env; coder containsValueForKey:cancels_key] {
        let cancels: bool = msg![env; coder decodeBoolForKey:cancels_key];
        env.objc.borrow_mut::<UIGestureRecognizerHostObject>(this).cancels_touches_in_view = cancels;
    }
    
    // Decode delays touches began
    let delays_began_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIDelaysTouchesBegan");
    if msg![env; coder containsValueForKey:delays_began_key] {
        let delays: bool = msg![env; coder decodeBoolForKey:delays_began_key];
        env.objc.borrow_mut::<UIGestureRecognizerHostObject>(this).delays_touches_began = delays;
    }
    
    // Decode delays touches ended
    let delays_ended_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIDelaysTouchesEnded");
    if msg![env; coder containsValueForKey:delays_ended_key] {
        let delays: bool = msg![env; coder decodeBoolForKey:delays_ended_key];
        env.objc.borrow_mut::<UIGestureRecognizerHostObject>(this).delays_touches_ended = delays;
    }
    
    this
}

- (UIGestureRecognizerState)state {
    env.objc.borrow::<UIGestureRecognizerHostObject>(this).state
}

- (())setEnabled:(bool)enabled {
    env.objc.borrow_mut::<UIGestureRecognizerHostObject>(this).enabled = enabled;
}

- (bool)isEnabled {
    env.objc.borrow::<UIGestureRecognizerHostObject>(this).enabled
}

- (id)view { // UIView
    env.objc.borrow::<UIGestureRecognizerHostObject>(this).view
}

- (())addTarget:(id)target action:(SEL)action {
    let host_obj = env.objc.borrow_mut::<UIGestureRecognizerHostObject>(this);
    host_obj.target = target;
    host_obj.action = if action.is_null() { None } else { Some(action) };
}

- (())removeTarget:(id)_target action:(SEL)_action {
    let host_obj = env.objc.borrow_mut::<UIGestureRecognizerHostObject>(this);
    host_obj.target = nil;
    host_obj.action = None;
}

- (CGPoint)locationInView:(id)_view { // UIView
    log_dbg!("UIGestureRecognizer {:?} locationInView: stub returning (0,0)", this);
    CGPoint { x: 0.0, y: 0.0 }
}

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

@implementation UITapGestureRecognizer: UIGestureRecognizer

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITapGestureRecognizerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return nil;
    }
    
    // Decode number of taps
    let taps_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UINumberOfTapsRequired");
    if msg![env; coder containsValueForKey:taps_key] {
        let taps: NSInteger = msg![env; coder decodeIntegerForKey:taps_key];
        env.objc.borrow_mut::<UITapGestureRecognizerHostObject>(this).number_of_taps_required = taps;
    }
    
    // Decode number of touches
    let touches_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UINumberOfTouchesRequired");
    if msg![env; coder containsValueForKey:touches_key] {
        let touches: NSInteger = msg![env; coder decodeIntegerForKey:touches_key];
        env.objc.borrow_mut::<UITapGestureRecognizerHostObject>(this).number_of_touches_required = touches;
    }
    
    this
}

- (())setNumberOfTapsRequired:(NSInteger)taps {
    env.objc.borrow_mut::<UITapGestureRecognizerHostObject>(this).number_of_taps_required = taps;
}

- (NSInteger)numberOfTapsRequired {
    env.objc.borrow::<UITapGestureRecognizerHostObject>(this).number_of_taps_required
}

- (())setNumberOfTouchesRequired:(NSInteger)touches {
    env.objc.borrow_mut::<UITapGestureRecognizerHostObject>(this).number_of_touches_required = touches;
}

- (NSInteger)numberOfTouchesRequired {
    env.objc.borrow::<UITapGestureRecognizerHostObject>(this).number_of_touches_required
}

@end

@implementation UISwipeGestureRecognizer: UIGestureRecognizer

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UISwipeGestureRecognizerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return nil;
    }
    
    // Decode direction
    let direction_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UISwipeGestureRecognizerDirection");
    if msg![env; coder containsValueForKey:direction_key] {
        let direction: NSInteger = msg![env; coder decodeIntegerForKey:direction_key];
        env.objc.borrow_mut::<UISwipeGestureRecognizerHostObject>(this).direction = direction;
    }
    
    // Decode number of touches
    let touches_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UINumberOfTouchesRequired");
    if msg![env; coder containsValueForKey:touches_key] {
        let touches: NSInteger = msg![env; coder decodeIntegerForKey:touches_key];
        env.objc.borrow_mut::<UISwipeGestureRecognizerHostObject>(this).number_of_touches_required = touches;
    }
    
    this
}

- (())setDirection:(NSInteger)direction {
    env.objc.borrow_mut::<UISwipeGestureRecognizerHostObject>(this).direction = direction;
}

- (NSInteger)direction {
    env.objc.borrow::<UISwipeGestureRecognizerHostObject>(this).direction
}

- (())setNumberOfTouchesRequired:(NSInteger)touches {
    env.objc.borrow_mut::<UISwipeGestureRecognizerHostObject>(this).number_of_touches_required = touches;
}

- (NSInteger)numberOfTouchesRequired {
    env.objc.borrow::<UISwipeGestureRecognizerHostObject>(this).number_of_touches_required
}

@end

@implementation UIPinchGestureRecognizer: UIGestureRecognizer

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIPinchGestureRecognizerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    this
}

- (f32)scale {
    env.objc.borrow::<UIPinchGestureRecognizerHostObject>(this).scale
}

- (())setScale:(f32)scale {
    env.objc.borrow_mut::<UIPinchGestureRecognizerHostObject>(this).scale = scale;
}

- (f32)velocity {
    env.objc.borrow::<UIPinchGestureRecognizerHostObject>(this).velocity
}

@end

@implementation UIPanGestureRecognizer: UIGestureRecognizer

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIPanGestureRecognizerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return nil;
    }
    
    // Decode minimum touches
    let min_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIMinimumNumberOfTouches");
    if msg![env; coder containsValueForKey:min_key] {
        let min: NSInteger = msg![env; coder decodeIntegerForKey:min_key];
        env.objc.borrow_mut::<UIPanGestureRecognizerHostObject>(this).minimum_number_of_touches = min;
    }
    
    // Decode maximum touches
    let max_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIMaximumNumberOfTouches");
    if msg![env; coder containsValueForKey:max_key] {
        let max: NSInteger = msg![env; coder decodeIntegerForKey:max_key];
        env.objc.borrow_mut::<UIPanGestureRecognizerHostObject>(this).maximum_number_of_touches = max;
    }
    
    this
}

- (CGPoint)translationInView:(id)_view { // UIView
    env.objc.borrow::<UIPanGestureRecognizerHostObject>(this).translation
}

- (())setTranslation:(CGPoint)translation inView:(id)_view { // UIView
    env.objc.borrow_mut::<UIPanGestureRecognizerHostObject>(this).translation = translation;
}

- (CGPoint)velocityInView:(id)_view { // UIView
    env.objc.borrow::<UIPanGestureRecognizerHostObject>(this).velocity
}

- (())setMinimumNumberOfTouches:(NSInteger)min {
    env.objc.borrow_mut::<UIPanGestureRecognizerHostObject>(this).minimum_number_of_touches = min;
}

- (NSInteger)minimumNumberOfTouches {
    env.objc.borrow::<UIPanGestureRecognizerHostObject>(this).minimum_number_of_touches
}

- (())setMaximumNumberOfTouches:(NSInteger)max {
    env.objc.borrow_mut::<UIPanGestureRecognizerHostObject>(this).maximum_number_of_touches = max;
}

- (NSInteger)maximumNumberOfTouches {
    env.objc.borrow::<UIPanGestureRecognizerHostObject>(this).maximum_number_of_touches
}

@end

@implementation UIRotationGestureRecognizer: UIGestureRecognizer

+ (id)allocWithZone:(NSZonePtr)_zone {
    let rotation_host = Box::new(UIPinchGestureRecognizerHostObject::default());
    env.objc.alloc_object(this, rotation_host, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    this
}

- (f32)rotation {
    env.objc.borrow::<UIPinchGestureRecognizerHostObject>(this).scale
}

- (())setRotation:(f32)rotation {
    env.objc.borrow_mut::<UIPinchGestureRecognizerHostObject>(this).scale = rotation;
}

- (f32)velocity {
    env.objc.borrow::<UIPinchGestureRecognizerHostObject>(this).velocity
}

@end

@implementation UILongPressGestureRecognizer: UIGestureRecognizer

+ (id)allocWithZone:(NSZonePtr)_zone {
    let long_press_host = Box::new(UIGestureRecognizerHostObject::default());
    env.objc.alloc_object(this, long_press_host, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    this
}

@end

};
