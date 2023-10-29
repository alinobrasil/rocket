#[macro_use]
extern crate rocket;

mod routes;
use routes::{check_data, fetch_data, hello_world};

mod types;
use types::TaskMap;

#[launch]
fn rocket() -> _ {
    println!("{}", "🚀 The server is ready to accept requests");

    rocket::build()
        .manage(TaskMap::default())
        .mount("/", routes![hello_world, fetch_data, check_data])
}
