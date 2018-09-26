extern crate rocket;
mod users;

use rocket::{Route, Catcher};
use rocket_contrib::{Json};

#[derive(Serialize)]
struct ErrorResponse {
    error: String
}

#[catch(404)]
fn not_found() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: String::from("Not Found")
    })
}

#[catch(500)]
fn server_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: String::from("Server Error")
    })
}

pub fn init_routes() -> Vec<Route> {
    routes![
        users::index,
        users::create,
        users::show,
        users::update,
        users::delete,
    ]
}

pub fn init_catchers() -> Vec<Catcher> {
    catchers![not_found, server_error]
}
