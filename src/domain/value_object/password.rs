
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

const PASSWORD_LENGTH: usize = 15;

#[derive(Debug)]
pub struct Password{
    value: String,
}


#[derive(Debug)]
pub enum PasswordError{
    PasswordIsEmpty,
    PasswordLength,
    HashingError
}


impl TryFrom<String> for Password{
    type Error = PasswordError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
           if value.is_empty(){
            Err(PasswordError::PasswordIsEmpty)
           }else if value.len() < PASSWORD_LENGTH{
            Err(PasswordError::PasswordLength)
           }else{

            let salt = SaltString::generate(&mut OsRng);

            // Argon2 with default params (Argon2id v19)
            let argon2 = Argon2::default();

            // Hash password to PHC string ($argon2id$v=19$...)
            let password_hash = argon2.hash_password(value.as_bytes(), &salt)
            .map_err(|s| return  PasswordError::HashingError)?.to_string();
            

            Ok(Self{
                value: password_hash
            })
           }
    }
}

impl From<Password> for String{
    fn from(value: Password) -> Self {
        value.value.to_string() 
    }
}


impl From<&Password> for String{
    fn from(value: &Password) -> Self {
        value.value.to_string()
    }
}