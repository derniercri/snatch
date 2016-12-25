extern crate hyper;

use std::sync::{Arc, Mutex};

pub mod client;
mod contentlength;
pub mod download;
pub mod http_version;

/// Represents a number of bytes, in `u64`.
type Byte = u64;
/// Represents a 'chunk', which is just a piece of bytes.
type Chunk = Vec<u8>;
/// Represents a list of chunks
type Chunks = Vec<Chunk>;
/// Represents a shared mutable reference of chunks
type SChunks = Arc<Mutex<Chunks>>;
