use rocket::serde::{Deserialize};

pub mod registrar;
mod password;

struct User {
    username: String
}


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UserRegistration {
    username: String,
    password: String
}

struct UserRegistrationInternal {
    username: String,
    password_hash: String,
    salt: String
}

impl UserRegistrationInternal {
    pub fn new(user_registration: &UserRegistration) -> Self {
        match argon_hash_password::create_hash_and_salt(user_registration.password.as_str()) {
            Ok((password_hash, salt)) => {
                UserRegistrationInternal { username: user_registration.username.clone(), password_hash, salt }
            }
            Err(err) => {
                panic!("Error creating hash and salt: {}", err);
            }
        }
    }
}