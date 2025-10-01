#[cfg(target_os = "android")]
use jni::objects::{JClass, JString};
#[cfg(target_os = "android")]
use jni::JNIEnv;

#[cfg(target_os = "android")]
use std::sync::{Mutex, OnceLock};

#[cfg(target_os = "android")]
struct PendingLaunch {
    path: String,
    _display_name: String,
}

#[cfg(target_os = "android")]
fn pending_launch_slot() -> &'static Mutex<Option<PendingLaunch>> {
    static SLOT: OnceLock<Mutex<Option<PendingLaunch>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

#[cfg(target_os = "android")]
fn set_pending_launch(path: String, display_name: String) {
    let storage = pending_launch_slot();
    if let Ok(mut guard) = storage.lock() {
        *guard = Some(PendingLaunch {
            path,
            _display_name: display_name,
        });
    } else {
        log!("Failed to acquire pending launch lock");
    }
}

#[cfg(target_os = "android")]
fn clear_pending_launch() {
    let storage = pending_launch_slot();
    if let Ok(mut guard) = storage.lock() {
        guard.take();
    } else {
        log!("Failed to clear pending launch state");
    }
}

#[cfg(target_os = "android")]
pub fn take_pending_launch_args() -> Option<Vec<String>> {
    let storage = pending_launch_slot();
    match storage.lock() {
        Ok(mut guard) => guard.take().map(|launch| vec![String::new(), launch.path]),
        Err(_) => {
            log!("Failed to lock pending launch state");
            None
        }
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_org_touchhle_android_TouchHLENative_prepareLaunch(
    env: JNIEnv,
    _class: JClass,
    path: JString,
    name: JString,
) {
    let path_res = env.get_string(&path);
    let name_res = env.get_string(&name);
    let (Ok(path), Ok(name)) = (path_res, name_res) else {
        log!("Failed to retrieve launch parameters from Java");
        return;
    };
    set_pending_launch(path.into(), name.into());
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_org_touchhle_android_TouchHLENative_clearLaunch(
    _env: JNIEnv,
    _class: JClass,
) {
    clear_pending_launch();
}
