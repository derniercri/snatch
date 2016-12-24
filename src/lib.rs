extern crate hyper;

mod contentlength;
pub mod http_version;
pub mod download;

/// Represents a byte, in `u64`.
type Byte = u64;
