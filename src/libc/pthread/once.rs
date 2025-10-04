/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `pthread_once`.

use crate::abi::{CallFromHost, GuestFunction};
use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{MutPtr, SafeRead};
use crate::Environment;

/// Magic number used in `PTHREAD_ONCE_INIT`. This is part of the ABI!
const MAGIC_ONCE: u32 = 0x30B1BCBA;

#[repr(C, packed)]
struct pthread_once_t {
    /// Magic number (must be [MAGIC_ONCE])
    magic: u32,
    /// Boolean marking whether this has been initialised yet. This seems to be
    /// initialized to zero.
    init: u32,
}
unsafe impl SafeRead for pthread_once_t {}

fn pthread_once(
    env: &mut Environment,
    once_control: MutPtr<pthread_once_t>,
    init_routine: GuestFunction, // void (*init_routine)(void)
) -> i32 {
    let pthread_once_t { magic, init } = env.mem.read(once_control);
    
    // Validate magic number with better error handling
    if magic != MAGIC_ONCE {
        log!(
            "Warning: pthread_once_t at {:?} has invalid magic number: expected 0x{:08X}, got 0x{:08X}",
            once_control,
            MAGIC_ONCE,
            magic
        );
        
        // Try to initialize the structure if it appears uninitialized
        if magic == 0 || init == 0 {
            log!("Attempting to initialize pthread_once_t structure...");
            let new_once = pthread_once_t {
                magic: MAGIC_ONCE,
                init: 0,
            };
            env.mem.write(once_control, new_once);
            // Now run the init routine
            log_dbg!(
                "pthread_once_t at {:?} initialized and running init routine {:?}",
                once_control,
                init_routine
            );
            let final_once = pthread_once_t {
                magic: MAGIC_ONCE,
                init: 0xFFFFFFFF,
            };
            env.mem.write(once_control, final_once);
            () = init_routine.call_from_host(env, ());
            log_dbg!("Init routine {:?} done", init_routine);
            return 0;
        }
        
        // If magic is wrong and it doesn't look uninitialized, this is an error
        log!("Error: pthread_once_t appears corrupted, skipping initialization");
        return -1; // EINVAL
    }
    
    match init {
        0 => {
            log_dbg!(
                "pthread_once_t at {:?} hasn't been run yet, running init routine {:?}",
                once_control,
                init_routine
            );
            let new_once = pthread_once_t {
                magic,
                init: 0xFFFFFFFF,
            };
            env.mem.write(once_control, new_once);
            () = init_routine.call_from_host(env, ());
            log_dbg!("Init routine {:?} done", init_routine);
        }
        0xFFFFFFFF => {
            log_dbg!(
                "pthread_once_t at {:?} has already been run, doing nothing",
                once_control
            );
        }
        _ => {
            log!(
                "Warning: pthread_once_t at {:?} has unexpected init value: 0x{:08X}",
                once_control,
                init
            );
            return -1;
        }
    };
    0 // success. TODO: return an error on failure?
}

pub const FUNCTIONS: FunctionExports = &[export_c_func!(pthread_once(_, _))];
