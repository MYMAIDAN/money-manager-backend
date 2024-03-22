use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct UserDTO {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String
}


