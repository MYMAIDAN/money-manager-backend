use uuid::Uuid;

#[derive(Debug)]
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