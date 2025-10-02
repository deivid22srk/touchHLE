/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The AddressBook framework.
//!
//! This is a minimal implementation that just provides the necessary constants.

use crate::dyld::{ConstantExports, HostConstant};

// ABPropertyID constants
const kABPersonEmailProperty: i32 = 4; // Standard email property ID

pub const CONSTANTS: ConstantExports = &[
    ("_kABPersonEmailProperty", HostConstant::I32(kABPersonEmailProperty)),
];
