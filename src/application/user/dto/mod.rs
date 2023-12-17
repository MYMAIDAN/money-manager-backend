use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct User {
    name: String,
    surname: String,
    email: String,
    password: String
}