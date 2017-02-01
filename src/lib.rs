extern crate hyper;
extern crate pbr;

use std::sync::{Arc, Mutex};

pub mod authorization;
pub mod client;
pub mod contentlength;
pub mod download;
pub mod http_version;
pub mod response;
pub mod write;

/// Represents a number of bytes, as `u64`.
pub type Bytes = u64;
/// Represents a 'chunk', which is just a piece of bytes.
type Chunk = Vec<u8>;
/// Represents a list of chunks
pub type Chunks = Vec<Chunk>;
/// Represents a shared mutable reference of chunks
pub type SChunks = Arc<Mutex<Chunks>>;
