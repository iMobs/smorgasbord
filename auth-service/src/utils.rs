use argon2::Config;
use rand::Rng;

pub fn hash_password(password: &str) -> argon2::Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config)
}

pub fn verify_password(hash: &str, password: &str) -> argon2::Result<bool> {
    argon2::verify_encoded(hash, password.as_bytes())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "test password 123";

        let hash = hash_password(password).unwrap();
        assert_ne!(password, hash);
        assert!(verify_password(&hash, password).unwrap());
    }
}
