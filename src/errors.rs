use std::error;
use std::fmt;
use std::ffi;
use std::io;
use std::result;
use std::path;

pub type Result<T> = result::Result<T, LibsolvError>;

#[derive(Debug)]
pub enum LibsolvError {

}


// impl From<LibsolvError> for String {
//     fn from(err: LibsolvError) -> Self {
//         match err {

//         }
//     }
// }

// impl fmt::Display for LibsolvError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }


