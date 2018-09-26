extern crate serde_derive;
extern crate diesel;

use models::*;
use db::{DbConn};
use diesel::{RunQueryDsl,QueryResult};
use diesel::prelude::*;
use rocket_contrib::{Json};

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
fn show(conn: DbConn, user_id: i32) -> QueryResult<Option<Json<User>>> {
    use schema::users::dsl::*;
    users.find(user_id)
        .get_result::<User>(&*conn)
        .optional()
        .map(|users_vec| match users_vec {
            Some(users_vec) => Some(Json(users_vec)),
            None => None
        })
}

#[patch("/users/<user_id>", data = "<user>")]
fn update(conn: DbConn, user_id: i32, user: Json<UserChanges>) -> QueryResult<Option<Json<User>>> {
    use schema::users::dsl::*;
    let user = user.0;
    diesel::update(users.find(user_id)).set(&user).execute(&*conn)?;
    users.find(user_id)
        .get_result::<User>(&*conn)
        .optional()
        .map(|user| match user {
            Some(user) => Some(Json(user)),
            None => None
        })
}

#[delete("/users/<user_id>")]
fn delete(conn: DbConn, user_id: i32) -> QueryResult<Option<Json<User>>> {
    use schema::users::dsl::*;
    let target = users.find(user_id);
    target
        .get_result::<User>(&*conn)
        .optional()
        .and_then(|user| {
            diesel::delete(target)
                .execute(&*conn)?;
            Ok(user.map(|user| Json(user)))
        })
}
