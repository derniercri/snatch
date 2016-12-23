use hyper::version::HttpVersion;

pub fn is_valid_http_version(v: HttpVersion) -> bool {
    v >= HttpVersion::Http11
}
