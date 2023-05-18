use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}

impl JwToken {
    pub fn get_key() -> String {
        let config = Config::new();
        let key_str = config.map.get("SECRET_KEY").unwrap().as_str().unwrap();
        return key_str.to_owned();
    }
    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();
        return token;
    }
    pub fn new(user_id: i32) -> Self {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES").unwrap().as_i64().unwrap();
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(minutes))
            .expect("valid timestamp")
            .timestamp();
        return JwToken {
            user_id,
            exp: expiration as usize,
        };
    }
    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let token_result = decode::<JwToken>(&token, &key, &Validation::default());
        match token_result {
            Ok(data) => return Ok(data.claims),
            Err(error) => {
                let message = format!("{}", error);
                return Err(message);
            }
        }
    }
}

#[derive(Debug)]
pub enum JwTokenError {
    Missing,
    Invalid,
    Expired,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwToken {
    type Error = JwTokenError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("token") {
            Some(data) => {
                let raw_token = data.to_string();
                let token_result = JwToken::from_token(raw_token);
                match token_result {
                    Ok(token) => return Outcome::Success(token),
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return Outcome::Failure((Status::BadRequest, JwTokenError::Expired));
                        }
                        return Outcome::Failure((Status::BadRequest, JwTokenError::Invalid));
                    }
                }
            }
            None => return Outcome::Failure((Status::BadRequest, JwTokenError::Missing)),
        }
    }
}

#[cfg(test)]
mod jwt_tests {

    use super::{Config, JwToken};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseFromTest {
        pub user_id: i32,
        pub exp_minutes: i32,
    }

    #[test]
    fn get_key() {
        assert_eq!(String::from("secret"), JwToken::get_key());
    }

    #[test]
    fn get_exp() {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES").unwrap().as_i64().unwrap();
        assert_eq!(120, minutes);
    }

    #[test]
    fn decode_incorrect_token() {
        let encoded_token: String = String::from("invalid_token");
        match JwToken::from_token(encoded_token) {
            Err(message) => assert_eq!("InvalidToken", message),
            _ => panic!("Incorrect token should not be able to be encoded"),
        }
    }

    #[test]
    fn encode_decode() {
        let test_token = JwToken::new(5);
        let encoded_token = test_token.encode();
        let new_token = JwToken::from_token(encoded_token).unwrap();
        assert_eq!(5, new_token.user_id);
    }
}
