use hyper::header::{Authorization, Basic, Headers, Scheme};
use std::fmt::{Display, Formatter, Result};

/// Enum for the different types of authorization required by a remote document.
#[derive(Clone)]
pub enum AuthorizationType {
    Basic,
    Digest,
    Unknown,
}

/// Trait to extend functionalities of the Headers type, from `hyper`
pub trait GetAuthorizationType {
    /// Function to get the authorization type (if any) of a remote document.
    /// The returned type is `Option<AuthorizationType>`.
    fn get_authorization_type(&self) -> Option<AuthorizationType>;
}

impl GetAuthorizationType for Headers {
    /// Function to get the `WWW-Authenticate` container, from a given header.
    /// This function returns an Option that contains a `AuthorizationType` type.
    fn get_authorization_type(&self) -> Option<AuthorizationType> {
        match self.get_raw("WWW-Authenticate") {
            Some(raw) => {
                let header_content = String::from_utf8(raw.get(0).unwrap().clone()).unwrap();
                let mut header_parts = header_content.split(" ");

                let auth_type = match header_parts.next() {
                    Some(part) => {
                        match part {
                            "Basic" => AuthorizationType::Basic,
                            "Digest" => AuthorizationType::Digest,
                            _ => AuthorizationType::Unknown,
                        }
                    }
                    None => return None,
                };
                Some(auth_type)
            }
            None => None,
        }
    }
}

impl Display for AuthorizationType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &AuthorizationType::Basic => write!(f, "Basic"),
            &AuthorizationType::Digest => write!(f, "Digest"),
            _ => write!(f, "Unknown"),
        }
    }
}

#[derive(Clone)]
pub struct AuthorizationHeaderFactory {
    authorization_type: AuthorizationType,
    username: String,
    password: Option<String>,
}

impl AuthorizationHeaderFactory {
    pub fn new(authorization_type: AuthorizationType,
               username: String,
               password: Option<String>)
               -> AuthorizationHeaderFactory {
        AuthorizationHeaderFactory {
            authorization_type: authorization_type,
            username: username,
            password: password,
        }
    }

    pub fn build_header(&self) -> Authorization<String> {
        match self.authorization_type {
            AuthorizationType::Basic => Authorization(format!("Basic {}", self)),
            _ => {
                epanic!(&format!("{} Authorization is not supported!",
                                 self.authorization_type))
            }
        }
    }
}

impl Display for AuthorizationHeaderFactory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.authorization_type {
            AuthorizationType::Basic => {
                let basic_auth = Basic {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
                basic_auth.fmt_scheme(f)
            }
            _ => {
                if let Some(ref passwd) = self.password {
                    write!(f, "{}:{}", self.username, passwd)
                } else {
                    write!(f, "{}", self.username)
                }
            }
        }
    }
}
