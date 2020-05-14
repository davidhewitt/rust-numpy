//! Low-Level bindings for NumPy C API.
//!
//! https://numpy.org/doc/stable/reference/c-api
#![allow(non_camel_case_types)]

use pyo3::ffi;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::null_mut;

fn get_numpy_api(module: &str, capsule: &str) -> *const *const c_void {
    let module = CString::new(module).unwrap();
    let capsule = CString::new(capsule).unwrap();
    unsafe {
        assert_ne!(
            ffi::Py_IsInitialized(),
            0,
            r"Numpy API is called before initializing Python!
Please make sure that you get gil, by `let gil = Python::acquire_gil();`"
        );
        let numpy = ffi::PyImport_ImportModule(module.as_ptr());
        assert!(!numpy.is_null(), "Failed to import numpy module");
        let capsule = ffi::PyObject_GetAttrString(numpy as _, capsule.as_ptr());
        assert!(!capsule.is_null(), "Failed to import numpy module");
        ffi::PyCapsule_GetPointer(capsule, null_mut()) as _
    }
}

// Define Array&UFunc APIs
macro_rules! impl_api {
    [ $offset:expr; $fname:ident ( $($arg:ident : $t:ty),* ) $( -> $ret:ty )* ] => {
        #[allow(non_snake_case)]
        pub unsafe fn $fname(&self, $($arg : $t), *) $( -> $ret )* {
            let fptr = self.get($offset)
                           as *const extern fn ($($arg : $t), *) $( -> $ret )*;
            (*fptr)($($arg), *)
        }
    }
}

pub mod array;
pub mod flags;
pub mod objects;
pub mod types;
pub mod ufunc;

pub use self::array::*;
pub use self::flags::*;
pub use self::objects::*;
pub use self::types::*;
pub use self::ufunc::*;
