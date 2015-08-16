extern crate libc;
use std::ptr;

use std::ffi::CStr;
use std::str;

use libc::c_int;
use libc::c_char;

#[repr(C)]
pub struct Dart_NativeArguments;
#[repr(C)]
pub struct Dart_Handle;

#[allow(non_camel_case_types)]
pub type Dart_NativeFunction = extern fn(*mut Dart_NativeArguments);
#[allow(non_camel_case_types)]
pub type Dart_NativeEntryResolver = extern fn(*mut Dart_Handle, c_int, *mut bool) -> Option<Dart_NativeFunction>;
#[allow(non_camel_case_types)]
pub type Dart_NativeEntrySymbol = extern fn(Dart_NativeFunction) -> *const c_char;


extern {
	fn Dart_SetNativeResolver(library: *mut Dart_Handle, resolver: Dart_NativeEntryResolver, symbol: Option<Dart_NativeEntrySymbol>) -> *mut Dart_Handle;
	fn Dart_Null() -> *mut Dart_Handle;
	fn Dart_StringToCString(str: *mut Dart_Handle, cstr: *mut*mut c_char) -> *mut Dart_Handle;
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn rust_extension_Init(parent_library: *mut Dart_Handle) -> *mut Dart_Handle {
	unsafe {
		Dart_SetNativeResolver(parent_library, resolve_name, None);
		Dart_Null()
	}
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn resolve_name(name: *mut Dart_Handle, argc: c_int, auto_setup_scope: *mut bool) -> Option<Dart_NativeFunction> {
	unsafe {
		*auto_setup_scope = true;
		let mut c_name: *mut c_char = ptr::null_mut::<c_char>();
		Dart_StringToCString(name, (&mut c_name as *mut*mut c_char));
		let r_name = str::from_utf8((CStr::from_ptr(c_name)).to_bytes()).unwrap();
		if r_name == "hello" {
			return Some(hello);
		}
	}
	None
}	

#[allow(unused_variables)]
#[no_mangle]
pub extern fn hello(arguments: *mut Dart_NativeArguments) {
	println!("hello from rust");
}
