#[derive(Debug)]
pub struct Name{
    value: String
}

impl From<String> for Name{
    fn from(value: String) -> Self {
        Self{ value: value }
    }
}


impl From<Name> for String{
    fn from(value: Name) -> Self {
        value.value.to_owned()
    }
}

impl From<&Name> for String{
    fn from(value: &Name) -> Self {
        value.value.to_owned()
    }
}

#[derive(Debug)]
pub struct Surname {
    value: String
}

impl From<String> for Surname{
    fn from(value: String) -> Self {
        Self{value: value}
    }
}


impl From<Surname> for String{
    fn from(value: Surname) -> Self {
        value.value.to_string()
    }
}

impl From<&Surname> for String{
    fn from(value: &Surname) -> Self {
        value.value.to_owned()
    }
}