use crate::diesel;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::DB;
use crate::json_serialization::login::Login;
use crate::json_serialization::login_response::LoginResponse;
use crate::jwt::JwToken;
use crate::models::user::user::User;
use crate::schema::users;

pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {
    let password = credentials.password.clone();
    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&db.connection)
        .unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }

    match users[0].verify(password) {
        true => {
            let user_id = users[0].clone().id;
            let token = JwToken::new(user_id);
            let raw_token = token.encode();
            let response = LoginResponse {
                token: raw_token.clone(),
            };
            // let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok()
                .append_header(("token", raw_token))
                .json(&response)
        }
        false => HttpResponse::Unauthorized().await.unwrap(),
    }
}
