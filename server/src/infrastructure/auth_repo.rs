use actix_web::{error, Error};
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct AuthRepo {
    pub secret: String,
}

impl AuthRepo {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    fn verify_timestamp(timestamp: i64, allow_range: Duration) -> Result<(), Error> {
        let server_timestamp = Utc::now().timestamp();
        if (server_timestamp - timestamp).abs() > allow_range.num_seconds() {
            return Err(error::ErrorUnauthorized(format!(
                "request timestamp ({timestamp}) is not in {allow_range} secs range of server timestamp ({server_timestamp})",
                allow_range = allow_range.num_seconds(),
            )));
        }
        Ok(())
    }

    fn hmac(&self, message: &str) -> Vec<u8> {
        // Create alias for HMAC-SHA256
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");

        mac.update(message.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        code_bytes[..].to_vec()
    }

    pub fn verify_message(
        &self,
        expected: &str,
        message: &str,
        timestamp: i64,
    ) -> Result<(), Error> {
        AuthRepo::verify_timestamp(timestamp, Duration::seconds(5))?;

        let code_bytes = self.hmac(message);
        let expected = hex::decode(expected).unwrap();
        if code_bytes[..] == expected[..] {
            Ok(())
        } else {
            Err(error::ErrorUnauthorized("request failed authorization"))
        }
    }

    pub fn sign_message(&self, message: &str) -> String {
        hex::encode(self.hmac(message))
    }
}
