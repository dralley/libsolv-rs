use crate::{Pool, Solvable, Id};
use libc::{c_uchar, c_int, c_uint};

extern "C" {
    pub fn e_pool_arch2color(pool: *mut Pool, arch: Id) -> c_uchar;
    pub fn e_pool_colormatch(pool: *mut Pool, s1: *mut Solvable, s2: *mut Solvable) -> c_int;
    pub fn e_pool_arch2score(pool: *mut Pool, arch: Id) -> c_uint;
}
