#[derive(Debug)]
pub struct Password{
    value: String,
}

impl From<String> for Password{
    fn from(value: String) -> Self {
        Self{ value: value }
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