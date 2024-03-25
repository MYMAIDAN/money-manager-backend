use serde::Serialize;

#[derive(Serialize)]
pub struct Error{
    error_code: String,
    message: String
}
    
impl Error{
    pub fn new(code: &str, message: &str) -> Self{
        Self { error_code: code.to_owned(), message: message.to_owned() }
    }
}