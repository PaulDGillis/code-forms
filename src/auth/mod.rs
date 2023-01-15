use std::{rc::Rc, future::{ready, Ready} };
use actix_web::{ dev::ServiceRequest, HttpMessage, FromRequest, error };
use actix_web_httpauth::extractors::bearer::BearerAuth;
use derive_more::{ Display, Error };
use serde::{ Deserialize, Serialize };
use jsonwebtoken::{ Algorithm, DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind };

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(username: String) -> Claims {
        Claims {
            sub: username,
            exp: (chrono::offset::Utc::now().timestamp() as usize) + 86400 // Now + 1 day
        }
    }

    pub fn encode(&self) -> String {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let key = secret.as_bytes();

        jsonwebtoken::encode(&Header::default(), self, &EncodingKey::from_secret(key)).expect("Oops")
    }
}

pub type AuthenticationInfo = Rc<Claims>;

pub async fn jwt_validate(req: ServiceRequest, creds: BearerAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let secret= std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = secret.as_bytes();

    let token = creds.token();
    let validation = Validation::new(Algorithm::HS256);
        
    let token_data = match jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };

    let parsed = token_data.claims;
    req.extensions_mut().insert::<AuthenticationInfo>(Rc::new(parsed));
    Ok(req)
}

pub struct Authenticated(AuthenticationInfo);

#[derive(Debug, Display, Error)]
pub struct AuthError {
    name: &'static str,
}

impl error::ResponseError for AuthError {}

impl FromRequest for Authenticated {
    type Error = AuthError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let value = req.extensions().get::<AuthenticationInfo>().cloned();
        let result = match value {
            Some(v) => Ok(Authenticated(v)),
            None => Err(AuthError { name: "Auth Failed" }),
        };
        ready(result)
    }
}

impl std::ops::Deref for Authenticated {
    type Target = AuthenticationInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}