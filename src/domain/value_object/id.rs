use uuid::Uuid;

#[derive(Debug,Clone)]
pub struct ID{
    value: uuid::Uuid
}

impl From<Uuid> for ID{
    fn from(value: Uuid) -> Self {
        Self{ value: value }
    }
}


impl From<ID> for Uuid{
    fn from(value: ID) -> Self {
        value.value.clone()
    }
}


impl From<&ID> for Uuid{
    fn from(value: &ID) -> Self {
        value.value.clone()
    }
}

impl ToString for ID{
    fn to_string(&self) -> String {
        format!("{}",self.value).to_string()
    }
}