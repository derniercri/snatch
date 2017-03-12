use hyper::header::{AcceptRanges, ContentLength, Headers};
use std::ops::Deref;

use Bytes;

// Traits to extend functionalities of the Headers type, from `hyper`

pub trait HeadersSupporting {
    /// Function to get the content length of a remote document.
    /// The returned type is `Option<Bytes>`.
    fn get_content_length(&self) -> Option<Bytes>;
    /// Function to know if the server supports the AcceptRanges option.
    /// This function returns a boolean value.
    fn support_partialcontent(&self) -> bool;
}

impl HeadersSupporting for Headers {
    /// Function to get the `content-length` container, from a given header.
    /// This function returns an Option that contains a `Bytes` type.
    fn get_content_length(&self) -> Option<Bytes> {
        if self.has::<ContentLength>() {
            return Some(*self.get::<ContentLength>().unwrap().deref());
        }
        None
    }
    /// Function to know if the server supports the AcceptRanges option.
    fn support_partialcontent(&self) -> bool {
        self.has::<AcceptRanges>()
    }
}

