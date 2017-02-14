#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;

extern crate rocket_contrib;
use rocket_contrib::JSON;

#[macro_use] extern crate serde_derive;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

extern crate dotenv;
use dotenv::dotenv;

use std::env;

mod models;

#[derive(Deserialize)]
struct HelloReq {
    pub payee_name: String,
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[post("/transactions", format = "application/json", data = "<req>")]
fn transactions(req: JSON<HelloReq>) -> JSON<Vec<models::Transaction>> {
    JSON(vec![])
}

fn main() {
    rocket::ignite().mount("/", routes![transactions]).launch();
}
