use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    //web::Data<PgConnection> allows us to be understood by actix, actix only understant request within this type : web::Extractor<T> like web::Data<T> web::Form<T> ...
) -> HttpResponse {
    /*Form here allows us to place informations for us in the FormData struct using the "application/x-www-form-urlencoded" method
     * that looks like this : "name=le%20guin&email=ursula_le_guin%40gmail.com"
     * so the name and email will automaticaly go in the right field */
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => return HttpResponse::Ok().finish(),

        Err(e) => {
            println!("Error querying the database {e}");
            return HttpResponse::InternalServerError().finish(); // Allows us to return a error 400 if an error occurs
        }
    }
}
