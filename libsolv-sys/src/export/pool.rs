use crate::{Pool, Solvable, Id};
use libc::{c_int, c_char};

extern "C" {
    pub fn e_pool_id2solvable(pool: *const Pool, p: Id) -> *mut Solvable;
    pub fn e_pool_solvable2id(pool: *const Pool, p: *const Solvable) -> Id;
    pub fn e_pool_solvid2str(pool: *mut Pool, p: Id) -> *const c_char;
    pub fn e_pool_match_nevr(pool: *mut Pool, s: *mut Solvable, d: Id) -> c_int;
    pub fn e_pool_whatprovides(pool: *mut Pool, d: Id) -> Id;
    pub fn e_pool_whatprovides_ptr(pool: *mut Pool, d: Id) -> *mut Id;
}
