use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;


#[derive(Debug,Clone, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let now = OffsetDateTime::now_utc();
        // 1分間の有効期限
        let exp = now + time::Duration::minutes(1);
        let exp = exp.unix_timestamp() as usize;
        let iat = now.unix_timestamp() as usize;
        Self { sub, iat, exp }
    }
}

fn encode_jwt(claim: Claims, secret: String) -> Result<String, jsonwebtoken::errors::Error> {
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

fn decode_jwt(token: &str, secret: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let mut validation = Validation::default();
    validation.leeway = 5;
    let decoded = jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())?;
    Ok(decoded.claims)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "some-secret";


    #[test]
    fn test_encode_decode_jwt() {
        let secret = SECRET.to_string();
        let claim = Claims::new("test".to_string());
        let token = encode_jwt(claim.clone(), secret.clone()).unwrap();
        let decoded_claim = decode_jwt(&token, secret).unwrap();

        assert_eq!(claim, decoded_claim);
    }

    #[test]
    fn test_expired_token() {
        let secret = SECRET.to_string();
        let claim = Claims::new("test".to_string());
        let exp = OffsetDateTime::now_utc() - time::Duration::minutes(2);
        let exp = exp.unix_timestamp() as usize;
        let iat = OffsetDateTime::now_utc().unix_timestamp() as usize;
        let expired_claim = Claims {
            sub: claim.sub,
            iat,
            exp,
        };
        let token = encode_jwt(expired_claim, secret.clone()).unwrap();
        let result = decode_jwt(&token, secret);

        assert!(result.is_err());
    }
}