use argonautica::{Error, Hasher, Verifier};
use lazy_static::lazy_static;

lazy_static! {
    static ref PASSWORD_SECRET_KEY: String =
        std::env::var("PASSWORD_SECRET_KEY").expect("Can't read PASSWORD_SECRET_KEY");
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .hash()
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .verify()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_hashing() {
        std::env::set_var("PASSWORD_SECRET_KEY", "test secret key");

        let password = "test password 123";

        let hash = hash_password(&password).unwrap();
        assert_ne!(password, hash);
        assert!(verify_password(&hash, &password).unwrap());
    }
}
