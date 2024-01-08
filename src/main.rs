mod user;
mod lobby;

#[macro_use]
extern crate rocket;

use std::io::Error;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;

use crate::user::registrar::{InMemoryRegistrar, LoginError, UserRegistrar};
use crate::user::UserRegistration;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/register", data = "<user_input>")]
async fn register_user(user_gateway: &State<Box<dyn UserRegistrar>>, user_input: Json<UserRegistration>) {
    user_gateway.save_user(user_input.into_inner());
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct LoginCredentials {
    username: String,
    password: String
}

#[post("/login", data = "<user_input>")]
async fn user_login(user_gateway: &State<Box<dyn UserRegistrar>>, user_input: Json<LoginCredentials>) -> Result<String, Status>{
    match user_gateway.login_user(user_input.into_inner()) {
        Ok(session_id) => {
            println!("Login successful. Session ID: {}", session_id);
            Ok(session_id)
        },
        Err(LoginError::InvalidCredentials(message)) => {
            eprintln!("Login failed due to invalid credentials. Message: {}", message);
            Err(Status::from_code(401).expect("Invalid credentials"))
        }
        Err(LoginError::OtherError(message)) => {
            eprintln!("Login failed due to another error. Message: {}", message);
            Err(Status::from_code(500).expect("Login failed, internal error"))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Box::new(InMemoryRegistrar::default()) as Box<dyn UserRegistrar>)
        .mount("/", routes![index])
        .mount("/user", routes![register_user, user_login])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_register() {
    }
}