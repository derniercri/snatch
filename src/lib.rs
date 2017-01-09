extern crate hyper;

use std::sync::{Arc, Mutex};

mod contentlength;
pub mod http_version;
pub mod write;


/// Represents a byte, in `u64`.
type Byte = u64;
/// Represents a 'chunk', which is just a piece of bytes.
type Chunk = Vec<u8>;
/// Represents a list of chunks
type Chunks = Vec<Chunk>;
/// Represents a shared mutable reference of chunks
pub type SChunks = Arc<Mutex<Chunks>>;