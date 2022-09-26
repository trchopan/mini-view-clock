use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct AuthRepo {
    pub secret: String,
}

impl AuthRepo {
    pub fn new(secret: String) -> Self {
        Self { secret }
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

    pub fn verify_message(&self, expected: &str, message: &str) -> bool {
        let code_bytes = self.hmac(message);
        let expected = hex::decode(expected).unwrap();
        code_bytes[..] == expected[..]
    }

    pub fn sign_message(&self, message: &str) -> String {
        hex::encode(self.hmac(message))
    }
}
