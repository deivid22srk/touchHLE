/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `cxxabi.h`
//!
//! Resources:
//! - [Itanium C++ ABI specification](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#dso-dtor-runtime-api)

use crate::abi::GuestFunction;
use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::MutVoidPtr;
use crate::Environment;

use crate::abi::CallFromHost;

fn __cxa_atexit(
    env: &mut Environment,
    func: GuestFunction, // void (*func)(void *)
    p: MutVoidPtr,
    d: MutVoidPtr,
) -> i32 {
    env.libc_state.cxx_atexit_handlers.push((func, p, d));
    0 // success
}

pub(super) fn __cxa_finalize(env: &mut Environment, d: MutVoidPtr) {
    // This is meant to only run the handlers associated with a particular dso,
    // but we don't support unloading dylibs, so we can just run them all.
    if d.is_null() {
        log!("Finalizing C++ destructors");
        for (func, p, _d) in std::mem::take(&mut env.libc_state.cxx_atexit_handlers) {
            let _: () = func.call_from_host(env, (p,));
        }
    }
}

fn ___objc_personality_v0(_env: &mut Environment) -> i32 {
    log!("TODO: ___objc_personality_v0 called (unimplemented)");
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(__cxa_atexit(_, _, _)),
    export_c_func!(__cxa_finalize(_)),
    (
        "___objc_personality_v0",
        &(___objc_personality_v0 as fn(&mut crate::Environment) -> _),
    ),
];
