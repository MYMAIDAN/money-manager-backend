use crate::domain::value_object::{ID, Name,Surname,Email,Password};

#[derive(Debug)]
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

    pub fn password(&self) -> &Password{
        &self.password
    }
}