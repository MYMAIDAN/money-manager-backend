
use crate::domain::entity::user::User;

pub enum UserErrors{
    USER_ALREADY_EXIST,
    USER_DOES_NOT_EXIST

}

pub trait UserRepository{
    fn create_user(&self) -> Result<(),UserErrors>;
    fn get_by_id(&self, id: u64 ) -> Result<User, UserErrors>;
}