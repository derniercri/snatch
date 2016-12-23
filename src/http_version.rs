use hyper::version::HttpVersion;

/// Trait used to check HTTP Version
///
/// This trait is used to validate that a given HTTP Version match specific need.
pub trait ValidateHttpVersion {
    /// Validate that the current HttpVersion is at least 1.1 to be able to download chunks.
    fn greater_than_http_11(&self) -> bool;
}

impl ValidateHttpVersion for HttpVersion {
    /// Check the given HttpVersion.
    ///
    /// This version should be at least 1.1 to allow chunks downloading.
    fn greater_than_http_11(&self) -> bool {
        self >= &HttpVersion::Http11
    }
}
