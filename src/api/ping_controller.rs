use actix_web::HttpResponse;

#[get("/ping")]
fn ping() -> HttpResponse {
    HttpResponse::Ok()
        .body("pong!".to_string())
}
