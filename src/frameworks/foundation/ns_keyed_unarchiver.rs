/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSKeyedUnarchiver` and deserialization of its object graph format.
//!
//! Resources:
//! - You can get a good intuitive grasp of how the format works just by staring
//!   at a pretty-print of a simple nib file from something that can parse
//!   plists, e.g. `plutil -p` or `println!("{:#?}", plist::Value::...);`.
//! - Apple's [Archives and Serializations Programming Guide](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Archiving/Articles/archives.html)

use super::ns_string::{from_rust_string, get_static_str, to_rust_string};
use super::{ns_array, ns_dictionary};
use crate::dyld::{ConstantExports, HostConstant};
use crate::frameworks::core_foundation::time::apple_epoch;
use crate::frameworks::core_graphics::{CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::{NSInteger, NSUInteger};
use crate::frameworks::uikit::ui_geometry::{
    CGPointFromString, CGRectFromString, CGSizeFromString,
};
use crate::mem::{ConstVoidPtr, GuestUSize, MutVoidPtr};
use crate::objc::{
    autorelease, id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject,
    NSZonePtr,
};
use crate::Environment;
use flate2::read::{GzDecoder, ZlibDecoder};
use nibarchive::{NIBArchive, ValueVariant};
use plist::{Dictionary, Error, Uid, Value};
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::time::SystemTime;
use zip::ZipArchive;

const MAX_ARCHIVE_PARSE_DEPTH: usize = 6;

pub const NSKeyedArchiveRootObjectKey: &str = "root";

pub const CONSTANTS: ConstantExports = &[(
    "_NSKeyedArchiveRootObjectKey",
    HostConstant::NSString(NSKeyedArchiveRootObjectKey),
)];

struct NSKeyedUnarchiverHostObject {
    plist: Dictionary,
    current_key: Option<Uid>,
    /// linear map of Uid => id
    already_unarchived: Vec<Option<id>>,
    /// Something responding to NSKeyedUnarchiverDelegate
    delegate: id,
}
impl HostObject for NSKeyedUnarchiverHostObject {}

fn convert_nibarchive_to_plist(slice: &[u8]) -> Result<Value, Error> {
    let nib = NIBArchive::from_bytes(slice).map_err(|e| {
        log!("NIBArchive parsing error: {:?}", e);
        Value::from_reader_xml(Cursor::new(b"")).unwrap_err()
    })?;

    let mut objects: Vec<Value> = Vec::new();
    objects.push(Value::String("$null".to_string()));

    let mut class_map: HashMap<String, usize> = HashMap::new();
    let mut nib_objects_uids: Vec<usize> = Vec::new();
    let mut nib_connections_uids: Vec<usize> = Vec::new();
    let mut nib_visible_windows_uids: Vec<usize> = Vec::new();

    for obj in nib.objects() {
        let class_name = obj.class_name(nib.class_names()).name();

        let class_uid = if let Some(&uid) = class_map.get(class_name) {
            uid
        } else {
            let uid = objects.len();
            class_map.insert(class_name.to_string(), uid);

            let mut class_dict = Dictionary::new();
            class_dict.insert(
                "$classname".to_string(),
                Value::String(class_name.to_string()),
            );
            let classes = vec![
                Value::String(class_name.to_string()),
                Value::String("NSObject".to_string()),
            ];
            class_dict.insert("$classes".to_string(), Value::Array(classes));
            objects.push(Value::Dictionary(class_dict));
            uid
        };

        let mut obj_dict = Dictionary::new();
        obj_dict.insert("$class".to_string(), Value::Uid(Uid::new(class_uid as u64)));

        for value in obj.values(nib.values()) {
            let key = value.key(nib.keys());
            let val = match value.value() {
                ValueVariant::Int8(v) => Value::Integer((*v as i64).into()),
                ValueVariant::Int16(v) => Value::Integer((*v as i64).into()),
                ValueVariant::Int32(v) => Value::Integer((*v as i64).into()),
                ValueVariant::Int64(v) => Value::Integer((*v).into()),
                ValueVariant::Bool(v) => Value::Boolean(*v),
                ValueVariant::Float(v) => Value::Real(*v as f64),
                ValueVariant::Double(v) => Value::Real(*v),
                ValueVariant::Data(v) => Value::Data(v.clone()),
                ValueVariant::Nil => {
                    obj_dict.insert(key.to_string(), Value::Uid(Uid::new(0)));
                    continue;
                }
                ValueVariant::ObjectRef(idx) => Value::Uid(Uid::new(*idx as u64 + 1)),
            };
            obj_dict.insert(key.to_string(), val);
        }

        let obj_uid = objects.len();
        objects.push(Value::Dictionary(obj_dict));

        if class_name == "UIRuntimeOutletConnection" || class_name == "UIRuntimeEventConnection" {
            nib_connections_uids.push(obj_uid);
        } else if class_name.starts_with("UI") && class_name.contains("Window") {
            nib_visible_windows_uids.push(obj_uid);
        } else {
            nib_objects_uids.push(obj_uid);
        }
    }

    let mut root = Dictionary::new();
    root.insert("$version".to_string(), Value::Integer(100000.into()));
    root.insert(
        "$archiver".to_string(),
        Value::String("NSKeyedArchiver".to_string()),
    );
    root.insert("$objects".to_string(), Value::Array(objects));

    let mut top = Dictionary::new();

    if !nib_objects_uids.is_empty() {
        let objects_uid = create_array_in_objects(&mut root, &nib_objects_uids);
        top.insert(
            "UINibObjectsKey".to_string(),
            Value::Uid(Uid::new(objects_uid)),
        );
        top.insert(
            "UINibTopLevelObjectsKey".to_string(),
            Value::Uid(Uid::new(objects_uid)),
        );
    }

    if !nib_connections_uids.is_empty() {
        let connections_uid = create_array_in_objects(&mut root, &nib_connections_uids);
        top.insert(
            "UINibConnectionsKey".to_string(),
            Value::Uid(Uid::new(connections_uid)),
        );
    }

    if !nib_visible_windows_uids.is_empty() {
        let windows_uid = create_array_in_objects(&mut root, &nib_visible_windows_uids);
        top.insert(
            "UINibVisibleWindowsKey".to_string(),
            Value::Uid(Uid::new(windows_uid)),
        );
    }

    root.insert("$top".to_string(), Value::Dictionary(top));

    Ok(Value::Dictionary(root))
}

fn create_array_in_objects(root: &mut Dictionary, uids: &[usize]) -> u64 {
    if let Some(Value::Array(objects)) = root.get_mut("$objects") {
        let ns_array_class_uid = objects.len() as u64;

        let mut ns_array_class_dict = Dictionary::new();
        ns_array_class_dict.insert(
            "$classname".to_string(),
            Value::String("NSArray".to_string()),
        );
        ns_array_class_dict.insert(
            "$classes".to_string(),
            Value::Array(vec![
                Value::String("NSArray".to_string()),
                Value::String("NSObject".to_string()),
            ]),
        );
        objects.push(Value::Dictionary(ns_array_class_dict));

        let array_obj_uid = objects.len() as u64;
        let mut array_dict = Dictionary::new();
        array_dict.insert(
            "$class".to_string(),
            Value::Uid(Uid::new(ns_array_class_uid)),
        );

        let uid_array: Vec<Value> = uids
            .iter()
            .map(|&uid| Value::Uid(Uid::new(uid as u64)))
            .collect();
        array_dict.insert("NS.objects".to_string(), Value::Array(uid_array));

        objects.push(Value::Dictionary(array_dict));
        array_obj_uid
    } else {
        0
    }
}

fn parse_value_from_slice(slice: &[u8]) -> Result<Value, Error> {
    if slice.is_empty() {
        return Value::from_reader_xml(Cursor::new(b""));
    }

    if slice.starts_with(b"NIBArchive") {
        log!("Detected NIBArchive format, converting to plist...");
        return convert_nibarchive_to_plist(slice);
    }

    if slice.starts_with(b"bplist") {
        Value::from_reader(Cursor::new(slice))
    } else if slice.starts_with(b"<?xml") || slice.starts_with(b"<plist") {
        Value::from_reader_xml(Cursor::new(slice))
    } else {
        Value::from_reader(Cursor::new(slice))
            .or_else(|binary_err| Value::from_reader_xml(Cursor::new(slice)).or(Err(binary_err)))
    }
}

fn parse_keyed_archive(slice: &[u8]) -> Result<Value, Error> {
    parse_keyed_archive_inner(slice, 0)
}

fn parse_keyed_archive_inner(slice: &[u8], depth: usize) -> Result<Value, Error> {
    let mut last_err = match parse_value_from_slice(slice) {
        Ok(value) => return Ok(value),
        Err(err) => err,
    };

    if depth >= MAX_ARCHIVE_PARSE_DEPTH {
        return Err(last_err);
    }

    if slice.starts_with(b"\x1f\x8b") {
        let mut decoder = GzDecoder::new(Cursor::new(slice));
        let mut decompressed = Vec::new();
        match decoder.read_to_end(&mut decompressed) {
            Ok(_) => match parse_keyed_archive_inner(&decompressed, depth + 1) {
                Ok(value) => return Ok(value),
                Err(err) => last_err = err,
            },
            Err(decode_err) => {
                log!(
                    "NSKeyedUnarchiver: failed to gunzip archive: {}",
                    decode_err
                );
            }
        }
    }

    if slice.starts_with(b"\x78\x9c") || slice.starts_with(b"\x78\xda") {
        let mut decoder = ZlibDecoder::new(Cursor::new(slice));
        let mut decompressed = Vec::new();
        match decoder.read_to_end(&mut decompressed) {
            Ok(_) => match parse_keyed_archive_inner(&decompressed, depth + 1) {
                Ok(value) => return Ok(value),
                Err(err) => last_err = err,
            },
            Err(decode_err) => {
                log!(
                    "NSKeyedUnarchiver: failed to inflate zlib archive: {}",
                    decode_err
                );
            }
        }
    }

    if slice.starts_with(b"PK\x03\x04") {
        if let Ok(mut archive) = ZipArchive::new(Cursor::new(slice)) {
            for index in 0..archive.len() {
                if let Ok(mut file) = archive.by_index(index) {
                    if !file.is_file() {
                        continue;
                    }
                    let mut decompressed = Vec::new();
                    if file.read_to_end(&mut decompressed).is_ok() {
                        match parse_keyed_archive_inner(&decompressed, depth + 1) {
                            Ok(value) => return Ok(value),
                            Err(err) => last_err = err,
                        }
                    }
                }
            }
        }
    }

    Err(last_err)
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSKeyedUnarchiver: NSCoder

+ (id)allocWithZone:(NSZonePtr)_zone { // struct _NSZone*
    let unarchiver = Box::new(NSKeyedUnarchiverHostObject {
        plist: Dictionary::new(),
        current_key: None,
        already_unarchived: Vec::new(),
        delegate: nil
    });
    env.objc.alloc_object(this, unarchiver, &mut env.mem)
}

+ (id)unarchiveObjectWithFile:(id)path { // NSString *
    let data: id = msg_class![env; NSData dataWithContentsOfFile:path];
    if data == nil {
        return nil;
    }
    msg![env; this unarchiveObjectWithData:data]
}

+ (id)unarchiveObjectWithData:(id)data { // NSData *
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initForReadingWithData:data];
    let root_key = get_static_str(env, NSKeyedArchiveRootObjectKey);
    let result: id = msg![env; new decodeObjectForKey:root_key];
    autorelease(env, result)
}

// TODO: other init methods.

- (id)initForReadingWithData:(id)data { // NSData *
    if data == nil {
        return nil;
    }

    let length: NSUInteger = msg![env; data length];
    let bytes: ConstVoidPtr = msg![env; data bytes];
    let slice = env.mem.bytes_at(bytes.cast(), length);

    let host_obj = env.objc.borrow_mut::<NSKeyedUnarchiverHostObject>(this);
    assert!(host_obj.already_unarchived.is_empty());
    assert!(host_obj.current_key.is_none());
    assert!(host_obj.plist.is_empty());

    if slice.is_empty() {
        log!("NSKeyedUnarchiver: Warning: Attempting to parse empty data");
        release(env, this);
        return nil;
    }

    let format_hint = if slice.len() >= 6 {
        if slice.starts_with(b"bplist") {
            "binary plist"
        } else if slice.starts_with(b"<?xml") {
            "XML plist"
        } else if slice.starts_with(b"\x1f\x8b") {
            "gzip compressed"
        } else if slice.starts_with(b"\x78\x9c") || slice.starts_with(b"\x78\xda") {
            "zlib compressed"
        } else if slice.starts_with(b"PK\x03\x04") {
            "zip archive"
        } else {
            "unknown format"
        }
    } else {
        "data too short"
    };

    let plist = match parse_keyed_archive(slice) {
        Ok(plist) => plist,
        Err(err) => {
            log!("NSKeyedUnarchiver: Failed to parse keyed archive (detected format: {}): {}", format_hint, err);
            log!("NSKeyedUnarchiver: Data length: {} bytes, first 16 bytes: {:?}",
                 slice.len(),
                 &slice[..slice.len().min(16)]);
            release(env, this);
            return nil;
        }
    };

    let Some(plist) = plist.into_dictionary() else {
        log!("NSKeyedUnarchiver: Warning: Root plist value is not a dictionary");
        release(env, this);
        return nil;
    };

    if plist.get("$version").and_then(|v| v.as_unsigned_integer()) != Some(100000) {
        log!("NSKeyedUnarchiver: Warning: Unexpected archive version: {:?}",
             plist.get("$version"));
        release(env, this);
        return nil;
    }

    if plist.get("$archiver").and_then(|v| v.as_string()) != Some("NSKeyedArchiver") {
        log!("NSKeyedUnarchiver: Warning: Unexpected archiver: {:?}",
             plist.get("$archiver"));
        release(env, this);
        return nil;
    }

    let Some(objects) = plist.get("$objects").and_then(|v| v.as_array()) else {
        log!("NSKeyedUnarchiver: Warning: $objects is missing or not an array");
        release(env, this);
        return nil;
    };

    let key_count = objects.len();

    host_obj.already_unarchived = vec![None; key_count];
    host_obj.plist = plist;

    this
}

- (())dealloc {
    let host_obj = borrow_host_obj(env, this);
    let already_unarchived = std::mem::take(&mut host_obj.already_unarchived);

    for &object in already_unarchived.iter().flatten() {
        release(env, object);
    }

    env.objc.dealloc_object(this, &mut env.mem)
}

// TODO: implement calls to delegate methods
// weak/non-retaining
- (())setDelegate:(id)delegate { // id<NSKeyedUnarchiverDelegate>
    let host_object = env.objc.borrow_mut::<NSKeyedUnarchiverHostObject>(this);
    host_object.delegate = delegate;
}
- (id)delegate {
    env.objc.borrow::<NSKeyedUnarchiverHostObject>(this).delegate
}

// These methods drive most of the decoding. They get called in two cases:
// - By the code that initiates the unarchival, e.g. UINib, to retrieve
//   top-level objects.
// - By the object currently being unarchived, i.e. something that had
//   `initWithCoder:` called on it, to retrieve objects from its scope.
// They are all from the NSCoder abstract class and they return default values
// if the key is unknown.

- (bool)decodeBoolForKey:(id)key { // NSString *
    get_value_to_decode_for_key(env, this, key)
        .is_some_and(|value| value.as_boolean().unwrap())
}

- (f64)decodeDoubleForKey:(id)key { // NSString *
    get_value_to_decode_for_key(env, this, key).map_or(
        0.0,
        |value| value.as_real().unwrap()
    )
}

- (f32)decodeFloatForKey:(id)key { // NSString *
    // TODO: Check bounds, raise NSRangeException if it doesn't fit
    get_value_to_decode_for_key(env, this, key).map_or(
        0.0,
        |value| value.as_real().unwrap()
    ) as f32
}

- (NSInteger)decodeIntegerForKey:(id)key { // NSString *
    // TODO: Check bounds, raise NSRangeException if it doesn't fit
    get_value_to_decode_for_key(env, this, key).map_or(
        0,
        |value| value.as_signed_integer().unwrap()
    ).try_into().unwrap()
}

- (i32)decodeIntForKey:(id)key { // NSString *
    // TODO: Check bounds, raise NSRangeException if it doesn't fit
    get_value_to_decode_for_key(env, this, key).map_or(
        0,
        |value| value.as_signed_integer().unwrap()
    ).try_into().unwrap()
}

- (i32)decodeInt32ForKey:(id)key { // NSString *
    // TODO: Check bounds, raise NSRangeException if it doesn't fit
    get_value_to_decode_for_key(env, this, key).map_or(
        0,
        |value| value.as_signed_integer().unwrap()
    ).try_into().unwrap()
}

- (i64)decodeInt64ForKey:(id)key { // NSString *
    get_value_to_decode_for_key(env, this, key).map_or(
        0,
        |value| value.as_signed_integer().unwrap()
    )
}

- (id)decodeObjectForKey:(id)key { // NSString*
    let value_opt = get_value_to_decode_for_key(env, this, key);
    let Some(value) = value_opt else {
        return nil;
    };
    let value_cloned = value.clone();
    let object = unarchive_value_owned(env, this, &value_cloned);

    if object == nil {
        nil
    } else {
        autorelease(env, object)
    }
}

- (bool)containsValueForKey:(id)key { // NSString*
    assert!(key != nil);
    get_value_to_decode_for_key(env, this, key).is_some()
}

// TODO: add more decode methods

// These come from a category in UIKit's UIGeometry.h
- (CGPoint)decodeCGPointForKey:(id)key { // NSString*
    let string: id = msg![env; this decodeObjectForKey:key];
    CGPointFromString(env, string)
}
- (CGSize)decodeCGSizeForKey:(id)key { // NSString*
    let string: id = msg![env; this decodeObjectForKey:key];
    CGSizeFromString(env, string)
}
- (CGRect)decodeCGRectForKey:(id)key { // NSString*
    let string: id = msg![env; this decodeObjectForKey:key];
    CGRectFromString(env, string)
}

@end

};

fn borrow_host_obj(env: &mut Environment, unarchiver: id) -> &mut NSKeyedUnarchiverHostObject {
    env.objc.borrow_mut(unarchiver)
}

fn get_value_to_decode_for_key(env: &mut Environment, unarchiver: id, key: id) -> Option<&Value> {
    let key = to_rust_string(env, key); // TODO: avoid copying string
    let host_obj = borrow_host_obj(env, unarchiver);
    let scope = match host_obj.current_key {
        Some(current_uid) => {
            &host_obj.plist["$objects"].as_array().unwrap()[current_uid.get() as usize]
        }
        None => &host_obj.plist["$top"],
    }
    .as_dictionary()
    .unwrap();
    scope.get(&key)
}

fn unarchive_value_owned(env: &mut Environment, unarchiver: id, value: &Value) -> id {
    if let Some(uid) = value.as_uid().copied() {
        let object = unarchive_key(env, unarchiver, uid);
        retain(env, object);
        return object;
    }

    match value {
        Value::String(s) => {
            let string_obj = from_rust_string(env, s.clone());
            if string_obj == nil {
                log!("Warning: Failed to create NSString during unarchive_value_owned for string: {:?}", s);
            }
            string_obj
        },
        Value::Integer(int) => {
            #[allow(clippy::clone_on_copy)]
            let int = int.clone();
            let number: id = msg_class![env; NSNumber alloc];
            if let Some(int64) = int.as_signed() {
                let longlong: i64 = int64;
                msg![env; number initWithLongLong:longlong]
            } else if let Some(uint64) = int.as_unsigned() {
                let ulonglong: u64 = uint64;
                msg![env; number initWithUnsignedLongLong:ulonglong]
            } else {
                unreachable!();
            }
        }
        Value::Real(real) => {
            let number: id = msg_class![env; NSNumber alloc];
            let double: f64 = *real;
            msg![env; number initWithDouble:double]
        }
        Value::Boolean(b) => {
            let number: id = msg_class![env; NSNumber alloc];
            let bool_value: bool = *b;
            msg![env; number initWithBool:bool_value]
        }
        Value::Data(data) => {
            let len: GuestUSize = data.len().try_into().unwrap();
            let guest_bytes: MutVoidPtr = env.mem.alloc(len);
            env.mem
                .bytes_at_mut(guest_bytes.cast(), len)
                .copy_from_slice(data);
            let nsdata: id = msg_class![env; NSData alloc];
            msg![env; nsdata initWithBytesNoCopy:guest_bytes length:len freeWhenDone:true]
        }
        Value::Array(array) => {
            let mut objects = Vec::with_capacity(array.len());
            for element in array {
                let object = unarchive_value_owned(env, unarchiver, element);
                objects.push(object);
            }
            ns_array::from_vec(env, objects)
        }
        Value::Dictionary(dict) => {
            let mut pairs = Vec::with_capacity(dict.len());
            for (key, val) in dict.iter() {
                let key_obj = from_rust_string(env, key.clone());
                if key_obj == nil {
                    log!("Warning: Failed to create key NSString for dictionary key: {:?}", key);
                    continue; // Skip this pair
                }
                let value_obj = unarchive_value_owned(env, unarchiver, val);
                pairs.push((key_obj, value_obj));
            }
            let dict_obj = ns_dictionary::dict_from_keys_and_objects(env, &pairs);
            for (key_obj, value_obj) in pairs {
                release(env, key_obj);
                release(env, value_obj);
            }
            dict_obj
        }
        Value::Date(date_val) => {
            let time: SystemTime = (*date_val).into();
            let interval = match time.duration_since(apple_epoch()) {
                Ok(duration) => duration.as_secs_f64(),
                Err(_) => 0.0,
            };
            let date: id = msg_class![env; NSDate alloc];
            msg![env; date initWithTimeIntervalSinceReferenceDate:interval]
        }
        Value::Uid(_) => unreachable!(),
        _ => {
            log!("Warning: Unhandled plist::Value variant in unarchive_value_owned");
            nil
        }
    }
}

/// The core of the implementation: unarchive something by its uid.
///
/// This is recursive in practice: the `initWithCoder:` messages sent by this
/// function will be received by objects which will then send
/// `decodeXXXWithKey:` messages back to the unarchiver, which will then call
/// this function (and so on).
///
/// The object returned is retained only by the archiver. Remember to retain and
/// possibly autorelease it as appropriate.
fn unarchive_key(env: &mut Environment, unarchiver: id, key: Uid) -> id {
    let host_obj = borrow_host_obj(env, unarchiver);
    if let Some(existing) = host_obj.already_unarchived[key.get() as usize] {
        return existing;
    }

    let objects = host_obj.plist["$objects"].as_array().unwrap();

    let item = &objects[key.get() as usize];
    let new_object = match item {
        // The most general kind of item: a dictionary that contains the info
        // needed to invoke `initWithCoder:` on a class implementing NSCoding.
        Value::Dictionary(dict) => {
            let class_key = dict["$class"].as_uid().copied().unwrap();
            let class;
            if let Some(existing) = host_obj.already_unarchived[class_key.get() as usize] {
                class = existing;
            } else {
                let class_dict = &objects[class_key.get() as usize];
                let class_dict = class_dict.as_dictionary().unwrap();

                let class_name = class_dict["$classname"].as_string().unwrap();

                class = {
                    // Use link_class instead of get_known_class
                    // to allow placeholders for unimplemented classes
                    // from NIBArchive files
                    let class_name = class_name.to_string();
                    log_dbg!("NSKeyedUnarchiver: Unarchiving object of class '{}'", class_name);
                    env.objc.link_class(&class_name, /* is_metaclass: */ false, &mut env.mem)
                };
                let host_obj = borrow_host_obj(env, unarchiver);

                host_obj.already_unarchived[class_key.get() as usize] = Some(class);
            };

            let host_obj = borrow_host_obj(env, unarchiver);
            let old_current_key = host_obj.current_key;
            host_obj.current_key = Some(key);

            let new_object: id = msg![env; class alloc];
            if new_object == nil {
                log!("Warning: Failed to allocate object of class during unarchiving at key {:?}", key);
                let host_obj = borrow_host_obj(env, unarchiver);
                host_obj.current_key = old_current_key;
                return nil;
            }

            // Verify the object has a host_object after allocation
            if env.objc.get_host_object(new_object).is_none() {
                log!("Warning: Allocated object {:?} has no host_object during unarchiving. This may cause crashes.", new_object);
            }

            let new_object: id = msg![env; new_object initWithCoder:unarchiver];
            if new_object == nil {
                log!("Warning: initWithCoder returned nil during unarchiving at key {:?}", key);
            }

            let host_obj = borrow_host_obj(env, unarchiver);
            host_obj.current_key = old_current_key;

            new_object
        }
        Value::String(s) => {
            let s = s.to_string();
            from_rust_string(env, s)
        }
        Value::Integer(int) => {
            #[allow(clippy::clone_on_copy)]
            let int = int.clone();
            // Similar logic to deserialize_plist()
            let number: id = msg_class![env; NSNumber alloc];
            // TODO: is this the correct order of preference? does it matter?
            if let Some(int64) = int.as_signed() {
                let longlong: i64 = int64;
                msg![env; number initWithLongLong:longlong]
            } else if let Some(uint64) = int.as_unsigned() {
                let ulonglong: u64 = uint64;
                msg![env; number initWithUnsignedLongLong:ulonglong]
            } else {
                unreachable!(); // according to plist crate docs
            }
        }
        _ => unimplemented!("Unarchive: {:#?}", item),
    };

    let host_obj = borrow_host_obj(env, unarchiver);
    if new_object != nil {
        host_obj.already_unarchived[key.get() as usize] = Some(new_object);
    } else {
        log_dbg!("Warning: Unarchived object at key {:?} is nil, not caching", key);
    }
    new_object
}

/// Shortcut for use by `[_touchHLE_NSArray initWithCoder:]`.
///
/// The objects are to be considered retained by the `Vec`.
pub fn decode_current_array(env: &mut Environment, unarchiver: id) -> Vec<id> {
    let keys = keys_for_key(env, unarchiver, "NS.objects");

    keys.into_iter()
        .map(|key| {
            let new_object = unarchive_key(env, unarchiver, key);
            // object is retained by the Vec
            retain(env, new_object)
        })
        .collect()
}

/// Shortcut for use by `[_touchHLE_NSMutableDictionary initWithCoder:]`.
///
/// Similar to `decode_current_array`, but for dictionaries.
/// The keys and objects are not retained!
pub fn decode_current_dict(env: &mut Environment, unarchiver: id) -> Vec<(id, id)> {
    let keys = keys_for_key(env, unarchiver, "NS.keys");
    let vals = keys_for_key(env, unarchiver, "NS.objects");
    log_dbg!("decode_current_dict: keys {:?}, vals {:?}", keys, vals);

    let keys: Vec<id> = keys
        .into_iter()
        .map(|key| unarchive_key(env, unarchiver, key))
        .collect();
    let vals: Vec<id> = vals
        .into_iter()
        .map(|val| unarchive_key(env, unarchiver, val))
        .collect();

    keys.into_iter().zip(vals).collect()
}

/// Shortcut for use by `[NSDate initWithCoder:]`.
pub fn decode_current_date(env: &mut Environment, unarchiver: id) -> id {
    let key = get_static_str(env, "NS.time");
    let timestamp = get_value_to_decode_for_key(env, unarchiver, key)
        .unwrap()
        .as_real()
        .unwrap();

    let date: id = msg_class![env; NSDate alloc];
    msg![env; date initWithTimeIntervalSinceReferenceDate:timestamp]
}

/// Shortcut for use by `[NSData initWithCoder:]`.
pub fn decode_current_data(env: &mut Environment, unarchiver: id, is_mutable: bool) -> id {
    let key = get_static_str(env, "NS.data");
    // TODO: avoid copying (twice!)
    let bytes = get_value_to_decode_for_key(env, unarchiver, key)
        .unwrap()
        .as_data()
        .unwrap()
        .to_vec();
    let len: GuestUSize = bytes.len().try_into().unwrap();
    let guest_bytes: MutVoidPtr = env.mem.alloc(len);
    env.mem
        .bytes_at_mut(guest_bytes.cast(), len)
        .copy_from_slice(bytes.as_slice());

    assert!(is_mutable); // TODO
    let data: id = msg_class![env; NSMutableData alloc];
    msg![env; data initWithBytesNoCopy:guest_bytes length:len freeWhenDone:true]
}

fn keys_for_key(env: &mut Environment, unarchiver: id, key: &str) -> Vec<Uid> {
    let host_obj = borrow_host_obj(env, unarchiver);
    let objects = host_obj.plist["$objects"].as_array().unwrap();
    let item = &objects[host_obj.current_key.unwrap().get() as usize];
    let keys = item.as_dictionary().unwrap()[key].as_array().unwrap();
    keys.iter()
        .map(|value| value.as_uid().copied().unwrap())
        .collect()
}
