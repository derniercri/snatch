use hyper::version::HttpVersion;

pub trait ValidateHttpVersion {
    /// Check the HttpVersion is at least 1.1 to enable chunks download.
    ///
    fn greater_than_http_11(&self) -> bool;
}

impl ValidateHttpVersion for HttpVersion {
    /// Check the given HttpVersion.
    ///
    /// This version should be at least 1.1 to allow chunks downloading.
    ///
    fn greater_than_http_11(&self) -> bool {
        self >= &HttpVersion::Http11
    }
}
