#[macro_use]
extern crate rocket;

use rocket_ws::{Stream, WebSocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
