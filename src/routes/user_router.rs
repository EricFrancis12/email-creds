use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::{
    models::user_model::{User, UserCreationRequest, UserUpdateRequest},
    routes::errors::APIError,
    services::db::Storage,
};

#[get("/users")]
pub async fn get_all_users(s: Data<Storage>) -> HttpResponse {
    match s.get_all(&s.user).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().json(APIError::from_err(err)),
    }
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(s: Data<Storage>, path: Path<(String,)>) -> HttpResponse {
    let user_id = path.into_inner().0;
    match s.get_one_by_id(&s.user, user_id.as_str()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(APIError::from_err(err)),
    }
}

#[post("/users")]
pub async fn insert_new_user(
    s: Data<Storage>,
    creation_req: Json<UserCreationRequest>,
) -> HttpResponse {
    match s
        .insert_new(&s.user, User::new_from(creation_req.into_inner()))
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(APIError::from_err(err)),
    }
}

#[patch("/users/{user_id}")]
pub async fn update_user_by_id(
    s: Data<Storage>,
    path: Path<(String,)>,
    update_req: Json<UserUpdateRequest>,
) -> HttpResponse {
    let user_id = path.into_inner().0;
    match s
        .update_one_by_id(&s.user, user_id.as_str(), update_req.into_inner().to_doc())
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().json(APIError::from_err(err)),
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user_by_id(s: Data<Storage>, path: Path<(String,)>) -> HttpResponse {
    let user_id = path.into_inner().0;
    match s.delete_one_by_id(&s.user, user_id.as_str()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().json(APIError::from_err(err)),
    }
}
