mod user;
mod lobby;

#[macro_use]
extern crate rocket;

use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;

use crate::user::registrar::{InMemoryRegistrar, UserRegistrar};
use crate::user::UserRegistration;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user/register", data = "<user_input>")]
async fn register_user(user_gateway: &State<Box<dyn UserRegistrar>>, user_input: Json<UserRegistration>) {
    user_gateway.save_user(user_input.into_inner());
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct LoginCredentials {
    username: String,
    password: String
}

#[post("/user/login", data = "<user_input>")]
async fn user_login(user_gateway: &State<Box<dyn UserRegistrar>>, user_input: Json<LoginCredentials>) {
    user_gateway.login_user(user_input.into_inner());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Box::new(InMemoryRegistrar::default()) as Box<dyn UserRegistrar>)
        .mount("/", routes![index])
        .mount("/", routes![register_user])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_register() {
    }
}