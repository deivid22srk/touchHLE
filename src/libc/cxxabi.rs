/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `cxxabi.h`
//!
//! Resources:
//! - [Itanium C++ ABI specification](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#dso-dtor-runtime-api)

use crate::abi::{CallFromHost, GuestFunction};
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

    state
        .dso_destructors
        .entry(dso_handle)
        .or_default()
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
            let _: () = func.call_from_host(env, (arg,));
        }
    }
}

// Exception handling personality routine for Objective-C
// This is part of the ARM EHABI (Exception Handling ABI)
// Resources:
// - ARM EHABI specification
// - Itanium C++ ABI for exception handling concepts
fn ___objc_personality_v0(
    env: &mut Environment,
    state: i32,       // _Unwind_State
    exception: u32,   // _Unwind_Exception*
    context: u32,     // _Unwind_Context*
) -> i32 {
    log_dbg!(
        "___objc_personality_v0(state={}, exception={:#x}, context={:#x})",
        state,
        exception,
        context
    );
    
    // ARM EHABI states
    const _US_VIRTUAL_UNWIND_FRAME: i32 = 0;
    const _US_UNWIND_FRAME_STARTING: i32 = 1;
    const _US_UNWIND_FRAME_RESUME: i32 = 2;
    
    // Return values for personality routine
    const _URC_CONTINUE_UNWIND: i32 = 8;
    const _URC_HANDLER_FOUND: i32 = 6;
    const _URC_INSTALL_CONTEXT: i32 = 7;
    
    match state {
        _US_VIRTUAL_UNWIND_FRAME => {
            // Phase 1: Search for handler
            // For now, we don't handle Objective-C exceptions properly
            // Just continue unwinding
            log_dbg!("  Phase 1: Virtual unwind frame - continuing unwind");
            _URC_CONTINUE_UNWIND
        }
        _US_UNWIND_FRAME_STARTING => {
            // Phase 2: Cleanup
            log_dbg!("  Phase 2: Unwind frame starting - continuing unwind");
            _URC_CONTINUE_UNWIND
        }
        _US_UNWIND_FRAME_RESUME => {
            // Resume unwinding
            log_dbg!("  Unwind frame resume - installing context");
            _URC_INSTALL_CONTEXT
        }
        _ => {
            log!(
                "Warning: ___objc_personality_v0 called with unknown state {}",
                state
            );
            _URC_CONTINUE_UNWIND
        }
    }
}

// Stub for dyld stub binder
// This is called by the dynamic linker's lazy binding stubs
fn dyld_stub_binder(env: &mut Environment) {
    log!("Warning: dyld_stub_binder called - this should have been handled by touchHLE's dyld implementation");
    log!("  This may indicate an issue with lazy symbol resolution.");
    // In a real implementation, this would perform the symbol binding
    // But in touchHLE, we handle this differently via SVC instructions
    // If we reach here, something went wrong with our linking
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(__cxa_atexit(_, _, _)),
    export_c_func!(__cxa_finalize(_)),
    export_c_func!(___objc_personality_v0(_, _, _)),
    export_c_func!(dyld_stub_binder()),
];
