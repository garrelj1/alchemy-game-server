use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::user::{User, UserRegistration};

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
        match self.credential_store.lock() {
            Ok(mut cred_store) => {
                let credentials = LoginCredentials {
                    password_hash: user.password_hash,
                    salt: user.salt
                };

                cred_store.passwords.insert(user.username.clone(), credentials);
            }
            Err(error) => {
                eprintln!("Error updating the credential store, {}", error);
            }
        }

        match self.users.lock() {
            Ok(mut users) => {
                users.push(User {username: user.username});
            }
            Err(error) => {
                eprintln!("Error adding user, {}", user.username);
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

