use crate::domain::value_object::Email;
use sqlx::{postgres::PgPool, Error};
use crate::domain::entity::user::User;
use std::sync::Arc;
use async_trait::async_trait;
use super::dao::user_dao::UserDaoModel;
use crate::application::repositories::{UserRepository,CreateUserError,GetUserError};
use tracing::{
    warn,
    info,
    instrument
};
#[derive(Debug,Clone)]
pub struct UserRepositoryImpl{
    db_pool: Arc<PgPool>
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self{
        Self { db_pool: pool.clone() }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl{

    #[instrument]
    async fn create_user(&self, user: &User) -> Result<(), CreateUserError> {
        let user_dao_model = UserDaoModel::from(user);
        let pg_query_result = sqlx::query(
            "INSERT INTO users ( id, name, surname, email, password)
            VALUES ($1, $2, $3, $4, $5)")
            .bind(user_dao_model.id)
            .bind(&user_dao_model.name)
            .bind(&user_dao_model.surname)
            .bind(&user_dao_model.email)
            .bind(&user_dao_model.password)
            .execute(self.db_pool.as_ref()).await;

        match pg_query_result {
            Ok(_) => {
                info!("User {:?} has been created.", user_dao_model);
                Ok(())
            },
            Err(err) => {
                warn!("Can create user {:?} with error from db {:?}", user_dao_model, err);
                Err(CreateUserError::Connection)
            }
            
        }
        
    }
    
    async fn get_by_email(&self, email: &Email) -> Result<User,GetUserError>{
        let res = sqlx::query_as!(
            UserDaoModel,
            "SELECT * FROM users 
            WHERE email = $1",
            email.to_string()
        ).fetch_one(self.db_pool.as_ref()).await;

        match res {
            Ok(val) => Ok(val.into()),
            Err(e) => 
            match e {
                Error::RowNotFound => Err(GetUserError::NotFound),
                _ => Err(GetUserError::Connection)
                
            }
        }

            
        
    }

    async fn get_by_id(&self, id: u64 ) -> Result<crate::domain::entity::user::User, GetUserError> {
        todo!()
    }
}