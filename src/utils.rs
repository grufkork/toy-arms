use std::str::Utf8Error;

pub use winapi::{
    shared::minwindef::BOOL, shared::minwindef::HINSTANCE, shared::minwindef::TRUE,
    um::consoleapi::AllocConsole, um::libloaderapi::DisableThreadLibraryCalls,
    um::wincon::FreeConsole, um::winnt::DLL_PROCESS_ATTACH,
};
use winapi::shared::minwindef::FARPROC;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

/// cast is a substitution of reinterpret_cast in C++.
/// * `$address` - address or variable you wanna cast.
/// * `$type` - type you want to cast $address into.
#[macro_export]
macro_rules! cast {
    // Value cast
    ($address:expr, $type:ident) => {
        $address as *mut $type
    };
}

/// create_entrypoint fully automates the process of making DllMain on your behalf.
/// * `function` - function you want to run in the newly created thread.
#[macro_export]
macro_rules! create_entrypoint {
    ($function:expr) => {
        #[no_mangle]
        #[allow(non_snake_cake)]
        extern "system" fn DllMain(h_module: $crate::HINSTANCE, dw_reason: u32, _: *const ::std::ffi::c_void, ) -> $crate::BOOL {
            if dw_reason == $crate::DLL_PROCESS_ATTACH {
                unsafe {
                    $crate::DisableThreadLibraryCalls(h_module);
                }
                ::std::thread::spawn(|| {
                    if cfg!(debug_assertions) {
                        unsafe { $crate::AllocConsole(); }
                    }
                    $function();
                    if cfg!(debug_assertions) {
                        unsafe { $crate::FreeConsole(); }
                    }
                });
            }
            $crate::TRUE
        }
    };
}

pub fn make_lpcstr(text: &str) -> *const i8 {
    format!("{}{}", text, "\0").as_ptr() as *const i8
}

pub unsafe fn get_module_function_address(module_name: &str, function_name: &str) -> FARPROC {
    GetProcAddress(GetModuleHandleA(make_lpcstr(module_name)), make_lpcstr(function_name))
}

pub unsafe fn read_null_terminated_string(base_address: usize) -> Result<String, Utf8Error> {
    let mut name: Vec<u8> = Vec::new();
    let mut i: isize = 0;
    loop {
        let char_as_u8 = *(base_address as *const u8).offset(i);
        if char_as_u8 == 0 {
            return Ok(std::str::from_utf8(&name[..])?.to_owned());
        }
        name.push(char_as_u8);
        i += 1;
    }
}