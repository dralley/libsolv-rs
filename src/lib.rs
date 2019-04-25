use libc;
use libsolv_sys;

#[cfg(feature = "ext")]
use libsolvext_sys;

pub mod errors;
pub mod chksum;
pub mod pool;
pub mod queue;
pub mod repo;
pub mod solver;
pub mod sys;
pub mod transaction;

mod ownership;

pub use libsolv_sys::{solv_knownid, Id};

#[cfg(feature = "ext")]
pub mod ext;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
