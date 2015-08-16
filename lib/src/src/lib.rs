use std::ffi::CStr;
use std::ptr;
use std::str;

extern crate libc;
use libc::c_int;
use libc::c_char;


#[repr(C)]
pub struct _Dart_NativeArguments;
#[repr(C)]
pub struct _Dart_Handle;

#[allow(non_camel_case_types)]
pub type Dart_NativeArguments = *const _Dart_NativeArguments;
#[allow(non_camel_case_types)]
pub type Dart_Handle = *const _Dart_Handle;

#[allow(non_camel_case_types)]
pub type Dart_NativeFunction = extern fn(Dart_NativeArguments);
#[allow(non_camel_case_types)]
pub type Dart_NativeEntryResolver = extern fn(Dart_Handle, c_int, *mut bool) -> Option<Dart_NativeFunction>;
#[allow(non_camel_case_types)]
pub type Dart_NativeEntrySymbol = extern fn(Dart_NativeFunction) -> *const c_char;


extern {
	fn Dart_SetNativeResolver(library: Dart_Handle, resolver: Dart_NativeEntryResolver, symbol: Option<Dart_NativeEntrySymbol>) -> Dart_Handle;
	fn Dart_Null() -> Dart_Handle;
	fn Dart_StringToCString(str: Dart_Handle, cstr: *const*mut c_char) -> Dart_Handle;
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn rust_extension_Init(parent_library: Dart_Handle) -> Dart_Handle {
	unsafe {
		Dart_SetNativeResolver(parent_library, resolve_name, None);
		Dart_Null()
	}
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn resolve_name(name: Dart_Handle, argc: c_int, auto_setup_scope: *mut bool) -> Option<Dart_NativeFunction> {
	unsafe {
		*auto_setup_scope = true;
	}
	let function_name: &str = unsafe {
		let c_name: *mut c_char = ptr::null_mut();
		Dart_StringToCString(name, &c_name);

		str::from_utf8(CStr::from_ptr(c_name).to_bytes()).unwrap()
	};
	if function_name == "hello" {
		return Some(hello);
	}
	None
}	

#[allow(unused_variables)]
#[no_mangle]
pub extern fn hello(arguments: Dart_NativeArguments) {
	println!("hello from rust");
}
