#[macro_use]
extern crate rocket;

mod routes;
use routes::hello_world;

#[launch]
fn rocket() -> _ {
    println!("{}", "🚀 The server is ready to accept requests");

    rocket::build().mount("/", routes![hello_world,])
}
