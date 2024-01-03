use rocket::serde::{Deserialize};

pub mod registrar;

struct User {
    username: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserRegistration {
    username: String,
    password_hash: String,
    salt: String
}

impl UserRegistration {
    pub fn new(username: String, password: String) -> Self {
        match argon_hash_password::create_hash_and_salt(&password) {
            Ok((password_hash, salt)) => {
                return UserRegistration { username, password_hash, salt};
            }
            Err(err) => {
                panic!("Error creating hash and salt: {}", err);
            }
        }
    }
}