#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;

use std::env;
use dotenv::dotenv;

mod db;
mod schema;
mod models;
mod routes;

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    rocket::ignite()
        .mount("/", routes::init_routes())
        .catch(routes::init_catchers())
        .manage(db::init_pool(db_url))
        .launch();
}
