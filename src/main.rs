#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate dotenv;

use rocket_contrib::{Json};
use std::env;
use dotenv::dotenv;

mod db;
mod schema;
use schema::users;
use db::{DbConn};
use diesel::{RunQueryDsl,QueryResult};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
struct User {
    id: i32,
    name: String
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
struct NewUser {
    name: String
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name="users"]
struct UserChanges {
    name: Option<String>
}

#[get("/users")]
fn index(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
    use schema::users::dsl::*;
    users
        .get_results::<User>(&*conn)
        .map(|user| Json(user))
}

#[post("/users", data = "<user>")]
fn create(conn: DbConn, user: Json<NewUser>) -> QueryResult<Json<User>> {
    use schema::users::dsl::*;
    no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

    let user = user.0;
    diesel::insert_into(users).values(&user).execute(&*conn)?;
    users.find(last_insert_rowid)
        .get_result::<User>(&*conn)
        .map(|user| Json(user))
}

#[get("/users/<user_id>")]
fn show(conn: DbConn, user_id: i32) -> QueryResult<Json<User>> {
    use schema::users::dsl::*;
    users.find(user_id)
        .get_result::<User>(&*conn)
        .map(|users_vec| Json(users_vec))
}

#[patch("/users/<user_id>", data = "<user>")]
fn update(conn: DbConn, user_id: i32, user: Json<UserChanges>) -> QueryResult<Json<User>> {
    use schema::users::dsl::*;
    let user = user.0;
    diesel::update(users.find(user_id)).set(&user).execute(&*conn)?;
    users.find(user_id)
        .get_result::<User>(&*conn)
        .map(|users_vec| Json(users_vec))
}

#[delete("/users/<user_id>")]
fn delete(conn: DbConn, user_id: i32) -> QueryResult<Json<User>> {
    use schema::users::dsl::*;
    let target = users.find(user_id);
    let user = target.get_result::<User>(&*conn)?;
    diesel::delete(target).execute(&*conn)?;
    Ok(Json(user))
}

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    rocket::ignite()
        .mount("/", routes![index, create, show, update, delete])
        .manage(db::init_pool(db_url))
        .launch();
}
