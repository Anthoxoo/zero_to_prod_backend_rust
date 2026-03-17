use actix_web::{HttpResponse, web};

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    /*Form here allows us to place informations for us in the FormData struct using the "application/x-www-form-urlencoded" method
     * that looks like this : "name=le%20guin&email=ursula_le_guin%40gmail.com"
     * so the name and email will automaticaly go in the right field */
    return HttpResponse::Ok().finish(); // TODO!
}
