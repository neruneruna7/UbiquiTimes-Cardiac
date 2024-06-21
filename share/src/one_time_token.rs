use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    discord_user_id: Option<u64>,
    slack_user_id: Option<String>,
}

impl Claims {
    fn new(sub: String, discord_user_id: Option<u64>, slack_user_id: Option<String>) -> Self {
        let now = OffsetDateTime::now_utc();
        // 1分間の有効期限
        let exp = now + time::Duration::minutes(1);
        let exp = exp.unix_timestamp() as usize;
        let iat = now.unix_timestamp() as usize;
        Self { sub, iat, exp, discord_user_id, slack_user_id }
    }

    pub fn new_on_discord(sub: String, discord_user_id: u64) -> Self {
        Self::new(sub, Some(discord_user_id), None)
    }

    pub fn new_on_slack(sub: String, slack_user_id: String) -> Self {
        Self::new(sub, None, Some(slack_user_id))
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
        let claim = Claims::new("test".to_string(), None, None);
        let token = encode_jwt(claim.clone(), secret.clone()).unwrap();
        let decoded_claim = decode_jwt(&token, secret).unwrap();

        assert_eq!(claim, decoded_claim);
    }

    #[test]
    fn test_expired_token() {
        let secret = SECRET.to_string();
        let claim = Claims::new("test".to_string(), None, None);
        let exp = OffsetDateTime::now_utc() - time::Duration::minutes(2);
        let exp = exp.unix_timestamp() as usize;
        let iat = OffsetDateTime::now_utc().unix_timestamp() as usize;

        let expired_claim = Claims {
            sub: claim.sub,
            iat,
            exp,
            discord_user_id: claim.discord_user_id,
            slack_user_id: claim.slack_user_id,
        };
        let token = encode_jwt(expired_claim, secret.clone()).unwrap();
        let result = decode_jwt(&token, secret);

        assert!(result.is_err());
    }
}
