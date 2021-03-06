#![cfg(windows)]
#![allow(bad_style)]
#![allow(unused_variables)]

#[macro_use]
extern crate lazy_static;

extern crate config;
extern crate text;

use std::path::{
    PathBuf
};
use std::env;
use std::mem;
use std::ptr;

use std::os::raw::{
    c_int,
    c_uint,
    c_ushort,
    c_void
};

///Retrieves configuration of Fie.
fn config_file_path() -> PathBuf {
    let mut result = env::current_exe().unwrap();

    result.pop();
    result.push("plugins");
    result.set_file_name("vn-text-trim.toml");

    result
}

lazy_static! {
    static ref cleaner: text::Cleaner = text::Cleaner::new(config::Config::from_file(&config_file_path()).unwrap());
}

const TA_PLUGIN_VERSION: c_uint = 0x0002;

#[no_mangle]
pub extern "C" fn TAPluginGetVersion(arg: *const c_void) -> c_uint {
    TA_PLUGIN_VERSION
}

#[repr(C)]
pub struct TAInfo {
	size: c_uint,
	version: c_uint
}

#[no_mangle]
pub extern "C" fn TAPluginInitialize(ta_info: *const TAInfo, arg: *const c_void) -> c_int {
    1
}

type wchar_t = c_ushort;
type size_t = usize;

extern "C" {
    fn wcslen(buf: *const wchar_t) -> size_t;
}

#[no_mangle]
pub extern "C" fn TAPluginModifyStringPreSubstitution(string: *const wchar_t) -> *mut wchar_t {
    use std::ffi::{OsString, OsStr};
    use std::os::windows::ffi::OsStringExt;
    use std::os::windows::ffi::OsStrExt;

    let string = unsafe { std::slice::from_raw_parts(string, wcslen(string)) };
    let string = OsString::from_wide(string);

    let string = match string.into_string() {
        Ok(string) => string,
        Err(_) => return ptr::null_mut()
    };

    if let Some(new_text) = cleaner.clean(&string) {
        let result: &OsStr = new_text.as_ref();
        let mut result = result.encode_wide().collect::<Vec<u16>>();
        result.push(0);
        let result_ptr = result.as_mut_ptr();

        mem::forget(result);
        result_ptr
    }
    else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn TAPluginFree(buffer: *mut c_void) {
    let buffer = buffer as *mut wchar_t;
    unsafe {
        let len = wcslen(buffer) + 1;
        let buffer = Vec::from_raw_parts(buffer, len, len);
    }
}
