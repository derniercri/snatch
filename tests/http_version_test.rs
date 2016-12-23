extern crate hyper;
extern crate snatch;

#[cfg(test)]
mod test_http_versions {
    use hyper::version::HttpVersion;
    use snatch::http_version::is_valid_http_version;

    #[test]
    fn test_version_09() {
        assert!(!is_valid_http_version(HttpVersion::Http09))
    }

    #[test]
    fn test_version_10() {
        assert!(!is_valid_http_version(HttpVersion::Http10))
    }

    #[test]
    fn test_version_11() {
        assert!(is_valid_http_version(HttpVersion::Http11))
    }

    #[test]
    fn test_version_20() {
        assert!(is_valid_http_version(HttpVersion::Http20))
    }
}
