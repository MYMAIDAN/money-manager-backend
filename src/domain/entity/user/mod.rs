use crate::domain::value_object::{ID, Name,Surname,Email,Password};

pub struct User{
    id: ID,   
    name: Name,
    surname: Surname,
    email: Email,
    password: Password 
}


impl User{
    pub fn new(id: ID, name: Name, surname: Surname, email: Email, password: Password) -> Self {
        Self {
            id,
            name,
            surname,
            email,
            password
        }
    }
    pub fn id(&self) -> &ID{
        &self.id
    }

    pub fn name(&self) -> &Name{
        &self.name
    }

    pub fn surname(&self) -> &Surname{
        &self.surname
    }

    pub fn email(&self) -> &Email{
        &self.email
    }
}