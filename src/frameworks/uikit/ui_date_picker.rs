/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIDatePicker`.

use super::ui_view::ui_control::UIControlHostObject;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

// UIDatePickerMode
type UIDatePickerMode = NSInteger;
#[allow(dead_code)]
const UI_DATE_PICKER_MODE_TIME: UIDatePickerMode = 0;
#[allow(dead_code)]
const UI_DATE_PICKER_MODE_DATE: UIDatePickerMode = 1;
#[allow(dead_code)]
const UI_DATE_PICKER_MODE_DATE_AND_TIME: UIDatePickerMode = 2;
#[allow(dead_code)]
const UI_DATE_PICKER_MODE_COUNT_DOWN_TIMER: UIDatePickerMode = 3;

pub struct UIDatePickerHostObject {
    superclass: UIControlHostObject,
    date_picker_mode: UIDatePickerMode,
    date: id,               // NSDate
    locale: id,             // NSLocale
    calendar: id,           // NSCalendar
    time_zone: id,          // NSTimeZone
    minimum_date: id,       // NSDate
    maximum_date: id,       // NSDate
    minute_interval: NSInteger,
    countdown_duration: f64,
}
impl_HostObject_with_superclass!(UIDatePickerHostObject);

impl Default for UIDatePickerHostObject {
    fn default() -> Self {
        UIDatePickerHostObject {
            superclass: Default::default(),
            date_picker_mode: UI_DATE_PICKER_MODE_DATE_AND_TIME,
            date: nil,
            locale: nil,
            calendar: nil,
            time_zone: nil,
            minimum_date: nil,
            maximum_date: nil,
            minute_interval: 1,
            countdown_duration: 0.0,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIDatePicker: UIControl

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIDatePickerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];
    if this == nil {
        return nil;
    }
    
    // Decode mode
    let mode_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIDatePickerMode");
    if msg![env; coder containsValueForKey:mode_key] {
        let mode: NSInteger = msg![env; coder decodeIntegerForKey:mode_key];
        env.objc.borrow_mut::<UIDatePickerHostObject>(this).date_picker_mode = mode;
    }
    
    // Decode date
    let date_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIDate");
    let date: id = msg![env; coder decodeObjectForKey:date_key];
    if date != nil {
        retain(env, date);
        env.objc.borrow_mut::<UIDatePickerHostObject>(this).date = date;
    } else {
        // Set to current date if not specified
        let current_date: id = msg_class![env; NSDate date];
        retain(env, current_date);
        env.objc.borrow_mut::<UIDatePickerHostObject>(this).date = current_date;
    }
    
    // Decode minimum date
    let min_date_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIMinimumDate");
    let min_date: id = msg![env; coder decodeObjectForKey:min_date_key];
    if min_date != nil {
        retain(env, min_date);
        env.objc.borrow_mut::<UIDatePickerHostObject>(this).minimum_date = min_date;
    }
    
    // Decode maximum date
    let max_date_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIMaximumDate");
    let max_date: id = msg![env; coder decodeObjectForKey:max_date_key];
    if max_date != nil {
        retain(env, max_date);
        env.objc.borrow_mut::<UIDatePickerHostObject>(this).maximum_date = max_date;
    }
    
    // Decode minute interval
    let interval_key = crate::frameworks::foundation::ns_string::get_static_str(env, "UIMinuteInterval");
    if msg![env; coder containsValueForKey:interval_key] {
        let interval: NSInteger = msg![env; coder decodeIntegerForKey:interval_key];
        env.objc.borrow_mut::<UIDatePickerHostObject>(this).minute_interval = interval;
    }
    
    this
}

- (())setDatePickerMode:(UIDatePickerMode)mode {
    env.objc.borrow_mut::<UIDatePickerHostObject>(this).date_picker_mode = mode;
}

- (UIDatePickerMode)datePickerMode {
    env.objc.borrow::<UIDatePickerHostObject>(this).date_picker_mode
}

- (())setDate:(id)date { // NSDate
    let host_obj = env.objc.borrow_mut::<UIDatePickerHostObject>(this);
    if host_obj.date != nil {
        release(env, host_obj.date);
    }
    if date != nil {
        retain(env, date);
    }
    host_obj.date = date;
}

- (id)date { // NSDate
    env.objc.borrow::<UIDatePickerHostObject>(this).date
}

- (())setDate:(id)date animated:(bool)_animated { // NSDate
    () = msg![env; this setDate:date]
}

- (())setLocale:(id)locale { // NSLocale
    let host_obj = env.objc.borrow_mut::<UIDatePickerHostObject>(this);
    if host_obj.locale != nil {
        release(env, host_obj.locale);
    }
    if locale != nil {
        retain(env, locale);
    }
    host_obj.locale = locale;
}

- (id)locale { // NSLocale
    env.objc.borrow::<UIDatePickerHostObject>(this).locale
}

- (())setCalendar:(id)calendar { // NSCalendar
    let host_obj = env.objc.borrow_mut::<UIDatePickerHostObject>(this);
    if host_obj.calendar != nil {
        release(env, host_obj.calendar);
    }
    if calendar != nil {
        retain(env, calendar);
    }
    host_obj.calendar = calendar;
}

- (id)calendar { // NSCalendar
    env.objc.borrow::<UIDatePickerHostObject>(this).calendar
}

- (())setTimeZone:(id)time_zone { // NSTimeZone
    let host_obj = env.objc.borrow_mut::<UIDatePickerHostObject>(this);
    if host_obj.time_zone != nil {
        release(env, host_obj.time_zone);
    }
    if time_zone != nil {
        retain(env, time_zone);
    }
    host_obj.time_zone = time_zone;
}

- (id)timeZone { // NSTimeZone
    env.objc.borrow::<UIDatePickerHostObject>(this).time_zone
}

- (())setMinimumDate:(id)date { // NSDate
    let host_obj = env.objc.borrow_mut::<UIDatePickerHostObject>(this);
    if host_obj.minimum_date != nil {
        release(env, host_obj.minimum_date);
    }
    if date != nil {
        retain(env, date);
    }
    host_obj.minimum_date = date;
}

- (id)minimumDate { // NSDate
    env.objc.borrow::<UIDatePickerHostObject>(this).minimum_date
}

- (())setMaximumDate:(id)date { // NSDate
    let host_obj = env.objc.borrow_mut::<UIDatePickerHostObject>(this);
    if host_obj.maximum_date != nil {
        release(env, host_obj.maximum_date);
    }
    if date != nil {
        retain(env, date);
    }
    host_obj.maximum_date = date;
}

- (id)maximumDate { // NSDate
    env.objc.borrow::<UIDatePickerHostObject>(this).maximum_date
}

- (())setMinuteInterval:(NSInteger)interval {
    env.objc.borrow_mut::<UIDatePickerHostObject>(this).minute_interval = interval;
}

- (NSInteger)minuteInterval {
    env.objc.borrow::<UIDatePickerHostObject>(this).minute_interval
}

- (())setCountDownDuration:(f64)duration {
    env.objc.borrow_mut::<UIDatePickerHostObject>(this).countdown_duration = duration;
}

- (f64)countDownDuration {
    env.objc.borrow::<UIDatePickerHostObject>(this).countdown_duration
}

- (())dealloc {
    let host_obj = env.objc.borrow::<UIDatePickerHostObject>(this);
    
    if host_obj.date != nil {
        release(env, host_obj.date);
    }
    if host_obj.locale != nil {
        release(env, host_obj.locale);
    }
    if host_obj.calendar != nil {
        release(env, host_obj.calendar);
    }
    if host_obj.time_zone != nil {
        release(env, host_obj.time_zone);
    }
    if host_obj.minimum_date != nil {
        release(env, host_obj.minimum_date);
    }
    if host_obj.maximum_date != nil {
        release(env, host_obj.maximum_date);
    }
    
    () = msg_super![env; this dealloc]
}

@end

};
