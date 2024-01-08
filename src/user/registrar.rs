use std::collections::HashMap;
use std::io;
use std::sync::{Arc, LockResult, Mutex};
use crate::LoginCredentials;
use crate::user::{User, UserRegistration, UserRegistrationInternal};

pub trait UserRegistrar: Send + Sync + 'static {
    fn save_user(&self, user: UserRegistration);
    fn login_user(&self, login_credentials: LoginCredentials) -> Result<String, LoginError>;
}

#[derive(Default)]
pub struct InMemoryRegistrar {
    users: Arc<Mutex<Vec<User>>>,
    credential_store: Arc<Mutex<CredentialStore>>
}

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials(String),
    OtherError(String),
}

impl UserRegistrar for InMemoryRegistrar {
    fn save_user(&self, user: UserRegistration) {
        let internal_registration = UserRegistrationInternal::new(&user);

        match self.credential_store.lock() {
            Ok(mut cred_store) => {
                let credentials = RegistrationLoginCredentials {
                    password_hash: internal_registration.password_hash,
                    salt: internal_registration.salt
                };

                println!("adding hash: {} and salt: {} to credentials map",
                         &credentials.password_hash, &credentials.salt);
                cred_store.passwords.insert(String::from(&internal_registration.username), credentials);
            }
            Err(error) => {
                eprintln!("Error updating the credential store, {}", error);
            }
        }

        match self.users.lock() {
            Ok(mut users) => {
                println!("adding user: {} to username registrar", internal_registration.username);
                users.push(User {username: internal_registration.username});
            }
            Err(error) => {
                eprintln!("Error adding user: {}, with error {}", user.username, error);
            }
        }
    }

    fn login_user(&self, login_credentials: LoginCredentials) -> Result<String, LoginError> {
        match self.credential_store.lock() {
            Ok(store) => {
                if let Some(credentials) = store.passwords.get(&login_credentials.username) {
                    match argon_hash_password::check_password_matches_hash(
                        login_credentials.password.as_str(),
                        credentials.password_hash.as_str(),
                        credentials.salt.as_str()
                    ) {
                        Ok(check) => {
                            match check {
                                true => Ok(argon_hash_password::gen_session_id()),
                                false => Err(LoginError::InvalidCredentials(String::from("Invalid password")))
                            }
                        },
                        Err(error) => Err(LoginError::OtherError(error.to_string()))
                    }
                } else {
                    Err(LoginError::InvalidCredentials(String::from("Invalid username")))
                }
            }
            Err(error) => Err(LoginError::OtherError(error.to_string()))
        }
    }
}

#[derive(Default)]
pub struct RegistrationLoginCredentials {
    password_hash: String,
    salt: String
}

#[derive(Default)]
struct CredentialStore {
    passwords: HashMap<String, RegistrationLoginCredentials>
}

