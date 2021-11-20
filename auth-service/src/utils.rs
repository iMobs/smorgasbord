use argon2::{self, Config};
use lazy_static::lazy_static;

lazy_static! {
    static ref PASSWORD_SALT: String =
        std::env::var("PASSWORD_SALT").expect("Can't read PASSWORD_SALT");
}

pub fn hash_password(password: &str) -> argon2::Result<String> {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &PASSWORD_SALT.as_bytes(), &config)
}

pub fn verify_password(hash: &str, password: &str) -> argon2::Result<bool> {
    argon2::verify_encoded(hash, password.as_bytes())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_hashing() {
        std::env::set_var("PASSWORD_SALT", "test secret key");

        let password = "test password 123";

        let hash = hash_password(&password).unwrap();
        assert_ne!(password, hash);
        assert!(verify_password(&hash, &password).unwrap());
    }
}
