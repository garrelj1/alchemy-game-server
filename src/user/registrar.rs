use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::user::{User, UserRegistration, UserRegistrationInternal};

pub trait UserRegistrar: Send + Sync + 'static {
    async fn save_user(&self, user: UserRegistration);
}

#[derive(Default)]
pub struct InMemoryRegistrar {
    users: Arc<Mutex<Vec<User>>>,
    credential_store: Arc<Mutex<CredentialStore>>
}

impl UserRegistrar for InMemoryRegistrar {
    async fn save_user(&self, user: UserRegistration) {
        let internal_registration = UserRegistrationInternal::new(&user);

        match self.credential_store.lock() {
            Ok(mut cred_store) => {
                let credentials = LoginCredentials {
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
}

#[derive(Default)]
struct LoginCredentials {
    password_hash: String,
    salt: String
}

#[derive(Default)]
struct CredentialStore {
    passwords: HashMap<String, LoginCredentials>
}

