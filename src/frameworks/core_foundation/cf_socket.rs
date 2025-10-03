/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFSocket`

use super::cf_allocator::{kCFAllocatorDefault, CFAllocatorRef};
use super::CFTypeRef;
use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::mem::{MutVoidPtr, Ptr};
use crate::Environment;

fn CFSocketCreate(
    _env: &mut Environment,
    allocator: CFAllocatorRef,
    protocol_family: i32,
    type_: i32,
    protocol: i32,
    flags: u32,
    callout: MutVoidPtr,
    context: MutVoidPtr,
) -> CFTypeRef {
    assert_eq!(allocator, kCFAllocatorDefault); // unimplemented
    log!(
        "TODO: CFSocketCreate({}, {}, {}, {}, {:?}, {:?}) -> NULL",
        protocol_family,
        type_,
        protocol,
        flags,
        callout,
        context
    );
    Ptr::null()
}

// CFStream property keys
pub const kCFStreamPropertyShouldCloseNativeSocket: &str =
    "kCFStreamPropertyShouldCloseNativeSocket";
pub const kCFStreamPropertySocketNativeHandle: &str = "kCFStreamPropertySocketNativeHandle";

pub const CONSTANTS: ConstantExports = &[
    (
        "_kCFStreamPropertyShouldCloseNativeSocket",
        HostConstant::NSString(kCFStreamPropertyShouldCloseNativeSocket),
    ),
    (
        "_kCFStreamPropertySocketNativeHandle",
        HostConstant::NSString(kCFStreamPropertySocketNativeHandle),
    ),
];

pub const FUNCTIONS: FunctionExports = &[export_c_func!(CFSocketCreate(_, _, _, _, _, _, _))];
