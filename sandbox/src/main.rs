use std::{thread::sleep, time::Duration};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

const SECRET: &str = "some-secret";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let now = OffsetDateTime::now_utc() - time::Duration::minutes(2);
        // let exp = now + time::Duration::minutes(1);
        let exp = now + time::Duration::seconds(1);
        let exp = exp.unix_timestamp() as usize;
        let iat = now.unix_timestamp() as usize;
        Self { sub, iat, exp }
    }
}

fn generate_jwt() -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new("subject".to_owned());
    println!("claims: {:?}", claims);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )?;

    Ok(token)
}

fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_ref());
    let validation = Validation::default();
    let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(decoded_token.claims)
}


fn main() {
    let jwt = generate_jwt().unwrap();
    println!("Generated JWT: {}", jwt);

    // sleep(Duration::from_secs(2));

    match decode_jwt(&jwt) {
        Ok(t) => println!("JWT is valid: {:?}", t),
        Err(e) => println!("Error validating JWT: {}", e),
    }
}

// 1718870951
// 1875000000