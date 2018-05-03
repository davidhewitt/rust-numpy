//! Low-Level binding for UFunc API
//!
//! https://docs.scipy.org/doc/numpy/reference/c-api.ufunc.html

use std::ops::Deref;
use std::os::raw::*;
use std::ptr::null_mut;

use pyo3::ffi;
use pyo3::ffi::{PyObject, PyTypeObject};
use pyo3::{ObjectProtocol, PyModule, PyResult, Python};

use super::objects::*;
use super::types::*;

/// Low-Level binding for UFunc API
/// https://docs.scipy.org/doc/numpy/reference/c-api.ufunc.html
///
/// Most of UFunc API is exposed as the related function of this module object.
pub struct PyUFuncModule {
    numpy: PyModule,
    api: *const *const c_void,
}

impl Deref for PyUFuncModule {
    type Target = PyModule;
    fn deref(&self) -> &Self::Target {
        &self.numpy
    }
}

/// Define UFunc API in PyUFuncModule
macro_rules! pyufunc_api {
    [ $offset:expr; $fname:ident ( $($arg:ident : $t:ty),* ) $( -> $ret:ty )* ] => {
#[allow(non_snake_case)]
pub unsafe fn $fname(&self, $($arg : $t), *) $( -> $ret )* {
    let fptr = self.api.offset($offset) as (*const extern fn ($($arg : $t), *) $( -> $ret )* );
    (*fptr)($($arg), *)
}
}} // pyufunc_api!

impl PyUFuncModule {
    /// Import `numpy.core.umath` to use UFunc API.
    pub fn import(py: Python) -> PyResult<Self> {
        let numpy = py.import("numpy.core.umath")?;
        let c_api = numpy.as_object().getattr(py, "_UFUNC_API")?;
        let api = unsafe {
            pyo3::ffi::PyCapsule_GetPointer(c_api.as_object().as_ptr(), null_mut())
                as *const *const c_void
        };
        Ok(Self {
            numpy: numpy,
            api: api,
        })
    }

    pub unsafe fn get_pyufunc_type(&self) -> *mut PyTypeObject {
        *self.api as *mut PyTypeObject
    }

    pyufunc_api![1; PyUFunc_FromFuncAndData(func: *mut PyUFuncGenericFunction, data: *mut *mut c_void, types: *mut c_char, ntypes: c_int, nin: c_int, nout: c_int, identity: c_int, name: *const c_char, doc: *const c_char, unused: c_int) -> *mut PyObject];
    pyufunc_api![2; PyUFunc_RegisterLoopForType(ufunc: *mut PyUFuncObject, usertype: c_int, function: PyUFuncGenericFunction, arg_types: *mut c_int, data: *mut c_void) -> c_int];
    pyufunc_api![3; PyUFunc_GenericFunction(ufunc: *mut PyUFuncObject, args: *mut PyObject, kwds: *mut PyObject, op: *mut *mut PyArrayObject) -> c_int];
    pyufunc_api![4; PyUFunc_f_f_As_d_d(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![5; PyUFunc_d_d(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![6; PyUFunc_f_f(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![7; PyUFunc_g_g(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![8; PyUFunc_F_F_As_D_D(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![9; PyUFunc_F_F(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![10; PyUFunc_D_D(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![11; PyUFunc_G_G(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![12; PyUFunc_O_O(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![13; PyUFunc_ff_f_As_dd_d(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![14; PyUFunc_ff_f(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![15; PyUFunc_dd_d(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![16; PyUFunc_gg_g(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![17; PyUFunc_FF_F_As_DD_D(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![18; PyUFunc_DD_D(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![19; PyUFunc_FF_F(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![20; PyUFunc_GG_G(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![21; PyUFunc_OO_O(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![22; PyUFunc_O_O_method(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![23; PyUFunc_OO_O_method(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![24; PyUFunc_On_Om(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![25; PyUFunc_GetPyValues(name: *mut c_char, bufsize: *mut c_int, errmask: *mut c_int, errobj: *mut *mut PyObject) -> c_int];
    pyufunc_api![26; PyUFunc_checkfperr(errmask: c_int, errobj: *mut PyObject, first: *mut c_int) -> c_int];
    pyufunc_api![27; PyUFunc_clearfperr()];
    pyufunc_api![28; PyUFunc_getfperr() -> c_int];
    pyufunc_api![29; PyUFunc_handlefperr(errmask: c_int, errobj: *mut PyObject, retstatus: c_int, first: *mut c_int) -> c_int];
    pyufunc_api![30; PyUFunc_ReplaceLoopBySignature(func: *mut PyUFuncObject, newfunc: PyUFuncGenericFunction, signature: *mut c_int, oldfunc: *mut PyUFuncGenericFunction) -> c_int];
    pyufunc_api![31; PyUFunc_FromFuncAndDataAndSignature(func: *mut PyUFuncGenericFunction, data: *mut *mut c_void, types: *mut c_char, ntypes: c_int, nin: c_int, nout: c_int, identity: c_int, name: *const c_char, doc: *const c_char, unused: c_int, signature: *const c_char) -> *mut PyObject];
    pyufunc_api![32; PyUFunc_SetUsesArraysAsData(data: *mut *mut c_void, i: usize) -> c_int];
    pyufunc_api![33; PyUFunc_e_e(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![34; PyUFunc_e_e_As_f_f(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![35; PyUFunc_e_e_As_d_d(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![36; PyUFunc_ee_e(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![37; PyUFunc_ee_e_As_ff_f(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![38; PyUFunc_ee_e_As_dd_d(args: *mut *mut c_char, dimensions: *mut npy_intp, steps: *mut npy_intp, func: *mut c_void)];
    pyufunc_api![39; PyUFunc_DefaultTypeResolver(ufunc: *mut PyUFuncObject, casting: NPY_CASTING, operands: *mut *mut PyArrayObject, type_tup: *mut PyObject, out_dtypes: *mut *mut PyArray_Descr) -> c_int];
    pyufunc_api![40; PyUFunc_ValidateCasting(ufunc: *mut PyUFuncObject, casting: NPY_CASTING, operands: *mut *mut PyArrayObject, dtypes: *mut *mut PyArray_Descr) -> c_int];
    pyufunc_api![41; PyUFunc_RegisterLoopForDescr(ufunc: *mut PyUFuncObject, user_dtype: *mut PyArray_Descr, function: PyUFuncGenericFunction, arg_dtypes: *mut *mut PyArray_Descr, data: *mut c_void) -> c_int];
}
