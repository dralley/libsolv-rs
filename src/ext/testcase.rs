use std::path::Path;
use crate::solver::Solver;
use libsolv_sys::Solver as _Solver;
use libsolv_sys::solv_free;
use crate::pool::PoolContext;
use crate::queue::Queue;
use std::ptr;
use std::ffi::CString;
use libc::{c_char, c_int, FILE};
use crate::ownership::SolvTake;
use crate::errors::LibsolvError;
use libc;

use crate::errors::*;

pub fn read<P>(pool: &PoolContext, path: P, job: &mut Queue) -> Result<(Solver, CString, c_int)>
    where P: AsRef<Path> {

    use libsolvext_sys::testcase_read;

    let fp: *mut FILE = ptr::null_mut();
    let mut resultp: *mut c_char = ptr::null_mut();
    let mut resultflagsp: c_int = 0;

    let path_str: &str = path.as_ref().to_str().expect(&format!("Cannot describe path {:?} as str", path.as_ref()));
    let testcase = CString::new(path_str).expect("invalid cstr");
    let solver: *mut _Solver = {
        let borrow = pool.borrow_mut();
        unsafe {testcase_read(borrow._p, fp, testcase.as_ptr(), &mut job._q, &mut resultp, &mut resultflagsp)}
    };

    let resultpString = unsafe {CString::solv_take_mut(&mut resultp)}?;

    //TODO: We left off here. Use path, not testcase. Check solver result
    Ok((Solver::new_with_solver(pool.clone_context(), solver), resultpString, resultflagsp))
}

pub fn solverresult(solver: &mut Solver, resultflags: c_int) -> Result<CString> {
    use libsolvext_sys::testcase_solverresult;

    unsafe {
        let _ = solver.ctx.borrow_mut();
        let mut resultstr = testcase_solverresult(solver._s, resultflags);
        CString::solv_take_mut(&mut resultstr)
    }
}

