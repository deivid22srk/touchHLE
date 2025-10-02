/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CAMediaTiming` protocol and related functions.

use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_foundation::time::{apple_epoch, CFTimeInterval};
use crate::Environment;
use std::time::SystemTime;

/// CACurrentMediaTime returns the current absolute time, in seconds.
/// This is based on mach_absolute_time() and is relative to an arbitrary
/// starting point. It's commonly used for animations and timing in Core
/// Animation.
fn CACurrentMediaTime(_env: &mut Environment) -> CFTimeInterval {
    // CACurrentMediaTime is similar to CFAbsoluteTimeGetCurrent but uses
    // a monotonic clock based on mach_absolute_time.
    // For compatibility, we use the same approach as CFAbsoluteTimeGetCurrent.
    SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64()
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CACurrentMediaTime()),
];
