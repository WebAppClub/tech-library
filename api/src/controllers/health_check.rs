use actix_web::HttpResponse;

// Request: GET /health_check HTTP/1.1
// Response: HTTP/1.1 200 OK, content-length: 0
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
