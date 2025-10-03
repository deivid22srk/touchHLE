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
use std::collections::HashMap;

/// Storage for C++ destructors registered via __cxa_atexit
#[derive(Default)]
pub struct State {
    /// Map from DSO handle to list of (destructor, argument) pairs
    dso_destructors: HashMap<u32, Vec<(GuestFunction, MutVoidPtr)>>,
}

impl State {
    fn get(env: &mut Environment) -> &mut Self {
        &mut env.libc_state.cxxabi
    }
}

fn __cxa_atexit(
    env: &mut Environment,
    func: GuestFunction, // void (*func)(void *)
    p: MutVoidPtr,
    d: MutVoidPtr,
) -> i32 {
    log_dbg!("__cxa_atexit({:?}, {:?}, {:?})", func, p, d);
    
    // Store destructor for later execution
    let dso_handle = d.to_bits();
    let state = State::get(env);
    
    state.dso_destructors
        .entry(dso_handle)
        .or_insert_with(Vec::new)
        .push((func, p));
    
    0 // success
}

fn __cxa_finalize(env: &mut Environment, d: MutVoidPtr) {
    log_dbg!("__cxa_finalize({:?})", d);
    
    let state = State::get(env);
    let dso_handle = d.to_bits();
    
    // Run destructors for this DSO in reverse order (LIFO)
    if let Some(destructors) = state.dso_destructors.remove(&dso_handle) {
        for (func, arg) in destructors.into_iter().rev() {
            log_dbg!("  Calling destructor {:?}({:?})", func, arg);
            
            // Call the destructor function
            // Note: We ignore errors here as we're in cleanup code
            let _ = func.call_from_host(env, &[arg.to_bits()]);
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
    export_c_func!(___objc_personality_v0()),
];
