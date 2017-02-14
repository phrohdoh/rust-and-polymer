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

mod schema;
mod models;

#[derive(Deserialize)]
struct PayeesReq {
    pub payee_name: String,
}

type PayeesResp = Vec<models::Payee>;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[post("/payees", format = "application/json", data = "<req>")]
fn api_payees(req: JSON<PayeesReq>) -> JSON<PayeesResp> {
    use schema::payees::dsl::*;

    let conn = establish_connection();
    let results = payees.filter(name.eq(&req.payee_name))
        .load::<models::Payee>(&conn)
        .expect("Error loading payees");

    JSON(results)
}

fn main() {
    rocket::ignite().mount("/", routes![api_payees]).launch();
}
