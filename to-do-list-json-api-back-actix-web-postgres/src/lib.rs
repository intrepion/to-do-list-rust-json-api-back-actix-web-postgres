#[cfg(test)]
mod tests {
    use super::index;
    use actix_web::http::header::ContentType;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn it_works() {
        let mut _app = test::init_service(App::new().service(index)).await;
        let _req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
    }
}

use actix_web::{get, Error, HttpResponse};

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}
