mod models;

use axum::{body::Body, extract::{Request, State}, http::{header::AUTHORIZATION, StatusCode}, middleware::Next, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{application::dtos::user::UserId, domain::value_object::ID, infrastructure::{error::Error, AppState}};

use self::models::TokenClaim;


pub async fn jwt_auth_middleware(
    coockie_jar: CookieJar,
    State(data): State<AppState>,
    mut req: Request<Body>,
    next: Next
) -> Result<impl IntoResponse, (StatusCode, Json<Error>)> {
    let token = coockie_jar.get("token")
    .map(|cockie| cockie.value().to_string())
    .or_else(||{
        req.headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value|{
            if auth_value.starts_with("Bearer "){
                Some(auth_value[7..].to_owned())
            }else{
                None
            }
        })
    });

    let token = token.ok_or_else(||{
        let json_error = Error::new("Fail",
        "You are not logged in, please  provide token");
        (StatusCode::UNAUTHORIZED,Json(json_error))
    })?;

    let claims = decode::<TokenClaim>(
        &token, 
        &DecodingKey::from_secret(data.settings.jwt.secret.as_ref()), 
        &Validation::default()
    ).map_err(|_|{
        let json_error =  Error::new("Fail",
    "Invalid token");
        (StatusCode::UNAUTHORIZED,Json(json_error))
    })?.claims;

    let user = uuid::Uuid::parse_str(&claims.sub).map_err(|_|{
        let json_error = Error::new("Fail",
        "Invalid token data");
        (StatusCode::UNAUTHORIZED,Json(json_error))
    
    })?;


    req.extensions_mut().insert(UserId(user.clone()));
    Ok(next.run(req).await)


}

pub fn create_token(sub: ID) -> TokenClaim{
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::hours(1)).timestamp() as usize;
    TokenClaim{
        iat: iat,
        sub: sub.to_string(),
        exp: exp
    }




}