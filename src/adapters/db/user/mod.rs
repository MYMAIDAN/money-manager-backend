mod models;
use crate::application::user::repository::*;

use super::DbPool;

pub struct UserRepositoryImpl{
    db: super::DbPool,    
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self{
        Self{ db: pool}
    }
}

impl UserRepository for UserRepositoryImpl{
    fn create_user(&self) -> Result<(),UserErrors> {
        let user: models::User;
        Ok(()) 
    }

    fn get_by_id(&self, id: u64 ) -> Result<crate::domain::entity::user::User, UserErrors> {
        todo!()
    }
}