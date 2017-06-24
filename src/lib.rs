extern crate libc;
extern crate libsolv_sys;

pub mod queue;
pub mod pool;
pub mod repo;
pub mod solver;
pub mod transaction;
pub mod ext;
mod ownership;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
