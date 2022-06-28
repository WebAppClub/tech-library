use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    // ## Request
    //     method: GET
    //     path: /health_check
    // ### query paramaeters
    //     Nothing
    // ## Response
    //     status: 200
    //     Body: None
    // ## Middleware
    //     Nothing

    HttpResponse::Ok().finish()
}
