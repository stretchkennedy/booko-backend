use schema::users;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct User {
    id: i32,
    name: String
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    name: String
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct UserChanges {
    name: Option<String>
}
