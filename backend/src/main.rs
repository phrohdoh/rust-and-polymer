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
struct TransactionByPayeeNameReq {
    pub payee_name: String,
}

type TransactionByPayeeNameResp = Vec<models::Transaction>;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn db_get_transactions_by_payee_name(ref_connection: &SqliteConnection, payee_name: &str) -> Vec<models::Transaction> {
    use schema::payees;
    use schema::transactions;

    transactions::table.filter(transactions::payee_id.eq_any(payees::table.select(payees::id).filter(payees::name.eq(payee_name))))
        .load::<models::Transaction>(ref_connection).unwrap_or(vec![])
}

#[post("/transactions_by_payee_name", format = "application/json", data = "<req>")]
fn api_transactions_by_payee_name(req: JSON<TransactionByPayeeNameReq>) -> JSON<TransactionByPayeeNameResp> {
    let conn = establish_connection();

    let results = db_get_transactions_by_payee_name(&conn, &req.payee_name);
    JSON(results)
}

fn main() {
    rocket::ignite().mount("/", routes![api_transactions_by_payee_name]).launch();
}
