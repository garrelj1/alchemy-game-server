mod user;
mod lobby;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;

use crate::user::registrar::{InMemoryRegistrar, UserRegistrar};
use crate::user::UserRegistration;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user/register", data = "<user_input>")]
async fn register_user(user_gateway: &State<&mut InMemoryRegistrar>, user_input: Json<UserRegistration>) {
    user_gateway.save_user(user_input.into_inner()).await;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(InMemoryRegistrar::default())
        .mount("/", routes![index])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_register() {
    }
}