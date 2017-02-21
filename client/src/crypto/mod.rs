
//! All crypto-related code.

use super::*;

pub type Secret = i64;
pub type Mask = i64;
pub type MaskedSecret = i64;
pub type Share = i64;

mod masking;
mod lss;
mod ae;

pub use self::masking::*;
pub use self::lss::*;
pub use self::ae::*;
