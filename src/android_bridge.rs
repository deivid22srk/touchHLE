#[cfg(target_os = "android")]
use base64::{engine::general_purpose::STANDARD, Engine};
#[cfg(target_os = "android")]
use jni::objects::{JClass, JString, JObjectArray};
#[cfg(target_os = "android")]
use jni::sys::{jobjectArray, jstring};
#[cfg(target_os = "android")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use png::{BitDepth, ColorType, Encoder};
#[cfg(target_os = "android")]
use serde::Serialize;
#[cfg(target_os = "android")]
use std::panic;
#[cfg(target_os = "android")]
use std::path::Path;
#[cfg(target_os = "android")]
use std::ptr;
#[cfg(target_os = "android")]
use std::sync::{Mutex, OnceLock};

#[cfg(target_os = "android")]
use crate::bundle::Bundle;
#[cfg(target_os = "android")]
use crate::fs::BundleData;
#[cfg(target_os = "android")]
use crate::image::Image;

#[cfg(target_os = "android")]
struct PendingLaunch {
    path: String,
    _display_name: String,
    option_args: Vec<String>,
}

#[cfg(target_os = "android")]
fn pending_launch_slot() -> &'static Mutex<Option<PendingLaunch>> {
    static SLOT: OnceLock<Mutex<Option<PendingLaunch>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

#[cfg(target_os = "android")]
fn set_pending_launch(path: String, display_name: String, option_args: Vec<String>) {
    let storage = pending_launch_slot();
    if let Ok(mut guard) = storage.lock() {
        *guard = Some(PendingLaunch {
            path,
            _display_name: display_name,
            option_args,
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
        Ok(mut guard) => guard.take().map(|launch| {
            let mut args = Vec::with_capacity(launch.option_args.len() + 2);
            args.push(String::new());
            args.extend(launch.option_args);
            args.push(launch.path);
            args
        }),
        Err(_) => {
            log!("Failed to lock pending launch state");
            None
        }
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_org_touchhle_android_TouchHLENative_prepareLaunch(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
    name: JString,
    option_args: jobjectArray,
) {
    let path_res = env.get_string(&path);
    let name_res = env.get_string(&name);
    let (Ok(path), Ok(name)) = (path_res, name_res) else {
        log!("Failed to retrieve launch parameters from Java");
        return;
    };

    let mut options = Vec::new();
    if !option_args.is_null() {
        let option_array = unsafe { JObjectArray::from_raw(option_args) };
        if let Ok(len) = env.get_array_length(&option_array) {
            for idx in 0..len {
                if let Ok(element) = env.get_object_array_element(&option_array, idx) {
                    let element = JString::from(element);
                    if let Ok(value) = env.get_string(&element) {
                        options.push(value.into());
                    }
                }
            }
        }
        let _ = option_array.into_raw();
    }

    set_pending_launch(path.into(), name.into(), options);
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_org_touchhle_android_TouchHLENative_clearLaunch(
    _env: JNIEnv,
    _class: JClass,
) {
    clear_pending_launch();
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_org_touchhle_android_TouchHLENative_inspectBundle(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jstring {
    let path_res = env.get_string(&path);
    let Ok(path_jni) = path_res else {
        log!("Failed to read bundle path from Java");
        return ptr::null_mut();
    };
    let path: String = path_jni.into();

    let json = match inspect_bundle_impl(&path) {
        Ok(json) => json,
        Err(err) => {
            log!("Failed to inspect bundle {path}: {err}");
            return ptr::null_mut();
        }
    };

    match env.new_string(json) {
        Ok(result) => result.into_raw(),
        Err(e) => {
            log!("Failed to create Java string: {e:?}");
            ptr::null_mut()
        }
    }
}

#[cfg(target_os = "android")]
#[derive(Serialize)]
struct AppInspection {
    display_name: String,
    version: String,
    bundle_identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_png: Option<String>,
}

#[cfg(target_os = "android")]
fn inspect_bundle_impl(path: &str) -> Result<String, String> {
    let bundle_path = Path::new(path);
    let bundle_data = BundleData::open_any(bundle_path)?;
    let (bundle, fs) = Bundle::new_bundle_and_fs_from_host_path(bundle_data, true)?;

    let display_name = panic::catch_unwind(|| bundle.display_name().to_owned())
        .unwrap_or_else(|_| bundle.bundle_name().to_owned());
    let version = panic::catch_unwind(|| bundle.bundle_version().to_owned())
        .unwrap_or_else(|_| "".to_owned());
    let bundle_identifier = panic::catch_unwind(|| bundle.bundle_identifier().to_owned())
        .unwrap_or_else(|_| "".to_owned());
    let minimum_os_version = bundle.minimum_os_version().map(|s| s.to_owned());

    let icon_png = match bundle.load_icon(&fs) {
        Ok(image) => match image_to_png_bytes(&image) {
            Ok(bytes) => Some(STANDARD.encode(bytes)),
            Err(e) => {
                log!("Failed to encode icon for {}: {}", path, e);
                None
            }
        },
        Err(e) => {
            log!("Failed to load icon for {}: {}", path, e);
            None
        }
    };

    let metadata = AppInspection {
        display_name,
        version,
        bundle_identifier,
        minimum_os_version,
        icon_png,
    };

    serde_json::to_string(&metadata).map_err(|e| e.to_string())
}

#[cfg(target_os = "android")]
fn image_to_png_bytes(image: &Image) -> Result<Vec<u8>, String> {
    let (width, height) = image.dimensions();
    let mut buffer = Vec::new();
    {
        let mut encoder = Encoder::new(&mut buffer, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|e| format!("Failed to write PNG header: {e}"))?;
        writer
            .write_image_data(image.pixels())
            .map_err(|e| format!("Failed to write PNG data: {e}"))?;
    }
    Ok(buffer)
}
