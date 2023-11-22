use actix_web::{get, post, web, HttpResponse, Scope};
use rbatis::crud;
use serde::{Deserialize, Serialize};

use crate::{
    db::{get_mysql_pool, get_rb},
    success,
};

pub fn routes() -> Scope {
    web::scope("/users")
        .service(add_user)
        .service(query_user)
        .service(update_user)
        .service(delete_user)
}

#[derive(Debug, Deserialize, Clone, Serialize, Default, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<i64>,
    pub user_name: Option<String>,
    pub password: Option<String>,
}

crud!(User {});

#[post("/add")]
pub async fn add_user(user: web::Json<User>) -> HttpResponse {
    let user = user.to_owned();
    // let id = save_user(user.clone()).await.unwrap();
    let id = save_user_rb(user).await;
    success(Some(id))
}

async fn save_user_rb(user: User) -> i64 {
    let rbatis = get_rb().await;
    let x = User::insert(&rbatis, &user).await;
    let x = x.unwrap();
    i64::from(x.last_insert_id)
}

#[allow(dead_code)]
async fn save_user(user: User) -> Result<u64, sqlx::Error> {
    let pool = get_mysql_pool().await;
    let sql = "insert into user(user_name, password) values (?,?)";
    let result = sqlx::query(sql)
        .bind(user.user_name)
        .bind(user.password)
        .execute(&pool)
        .await?;
    let id = result.last_insert_id();
    Ok(id)
}

#[post("/update/{id}")]
pub async fn update_user(id: web::Path<i64>, user: web::Json<User>) -> HttpResponse {
    let user = user.to_owned();

    let sql = "update user set user_name =?, password =? where id = ?";
    sqlx::query(sql)
        .bind(user.user_name)
        .bind(user.password)
        .bind(id.into_inner())
        .execute(&get_mysql_pool().await)
        .await
        .unwrap();
    // let _ = User::update_by_column(&get_rb().await, &user, "id").await;
    success(Some(1))
}

#[get("/query/{id}")]
pub async fn query_user(id: web::Path<i64>) -> HttpResponse {
    // let sql = "select * from user where id = ?";
    // let result = sqlx::query_as::<_, User>(sql)
    //     .bind(id.into_inner())
    //     .fetch_optional(&get_mysql_pool().await)
    //     .await
    //     .unwrap();
    let result = User::select_by_column(&get_rb().await, "id", id.into_inner())
        .await
        .unwrap();
    success(Some(result))
}

#[post("/delete/{id}")]
pub async fn delete_user(id: web::Path<i64>) -> HttpResponse {
    // let sql = "delete from user where id = ?";
    // sqlx::query(sql)
    //     .bind(id.into_inner())
    //     .execute(&get_mysql_pool().await)
    //     .await
    //     .unwrap();
    let _ = User::delete_by_column(&get_rb().await, "id", id.into_inner()).await;
    success(Some(1))
}
