
#[derive(Debug)]
pub struct Email{
    value: String
}

impl From<String> for Email{
    fn from(value: String) -> Self {
        Self { value: value }
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.value.to_owned()
    }
}


impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.value.to_string()
    }
}

impl From<&Email> for String {
    fn from(value: &Email) -> Self {
        value.value.to_string()
    }
}

