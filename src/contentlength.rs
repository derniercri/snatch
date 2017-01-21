use hyper::header::{ContentLength, Headers};
use std::ops::Deref;

use Bytes;

/// Trait to extend functionalities of the Headers type, from `hyper`
pub trait GetContentLength {
    /// Function to get the content length of a remote document.
    /// The returned type is `Option<Bytes>`.
    fn get_content_length(&self) -> Option<Bytes>;
}

impl GetContentLength for Headers {
    /// Function to get the `content-length` container, from a given header.
    /// This function returns an Option that contains a `Bytes` type.
    fn get_content_length(&self) -> Option<Bytes> {
        if self.has::<ContentLength>() {
            return Some(*self.get::<ContentLength>().unwrap().deref());
        }
        None
    }
}
