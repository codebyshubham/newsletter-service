use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let find_user_result = sqlx::query!("SELECT email FROM subscriptions where email = $1", form.email)
        .fetch_one(db_pool.get_ref())
        .await;

    if let Ok(find_user) = find_user_result {
        if !find_user.email.is_empty() {
            println!("User is already subscribed!");
            return HttpResponse::BadRequest().body("User is already subscribed!");
        }
    }

    let result = sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now()
    )
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}