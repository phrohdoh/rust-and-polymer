#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;

extern crate rocket_contrib;
use rocket_contrib::JSON;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct NamedObj {
    name: String,
}

#[derive(Serialize)]
struct HelloResp {
    msg: String,
}

#[post("/hello", format = "application/json", data = "<named>")]
fn hello(named: JSON<NamedObj>) -> JSON<HelloResp> {
    let resp = HelloResp { msg: format!("Hello, {}!", named.name) };
    JSON(resp)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
