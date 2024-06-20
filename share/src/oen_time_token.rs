use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

const SECRET: &str = "some-secret";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: OffsetDateTime,
    exp: OffsetDateTime,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let now = OffsetDateTime::now_utc();
        let exp = now + time::Duration::minutes(1);
        let iat = now;
        Self { sub, iat, exp }
    }
}

fn generate_jwt() -> Result<String, jsonwebtoken::errors::Error> {
    let now = OffsetDateTime::now_utc();
    let exp = now + time::Duration::minutes(1);

    let claims = Claims::new("subject".to_owned());

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )?;

    Ok(token)
}

fn valid() {
    match generate_jwt() {
        Ok(token) => println!("Generated JWT: {}", token),
        Err(e) => println!("Error generating JWT: {}", e),
    }
}