#[cfg(feature = "alloc")]
pub use alloc::collections::*;

#[cfg(feature = "alloc")]
pub mod hash_map;

#[cfg(feature = "alloc")]
pub use self::hash_map::HashMap;