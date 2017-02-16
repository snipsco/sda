
//! All crypto-related code.

use super::*;

pub type Secret = i64;
pub type Share = i64;
pub type Encryption = Vec<u8>;

mod lss;
mod ae;

pub use self::lss::*;
pub use self::ae::*;
