use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: String,
    pub comments: String
}
