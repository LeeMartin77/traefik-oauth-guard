#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello friend")
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}