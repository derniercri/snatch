use hyper::version::HttpVersion;

/// Check the given HttpVersion.
///
/// This version should be at least 1.1 to allow chunks downloading.
///
/// # Examples
///
/// ```
/// extern crate hyper;
/// extern crate snatch;
///
/// fn main() {
///     use hyper::version::HttpVersion;
///     use snatch::http_version::is_valid_http_version;
///
///     is_valid_http_version(HttpVersion::Http11);
/// }
/// ```
///
pub fn is_valid_http_version(v: HttpVersion) -> bool {
    v >= HttpVersion::Http11
}
