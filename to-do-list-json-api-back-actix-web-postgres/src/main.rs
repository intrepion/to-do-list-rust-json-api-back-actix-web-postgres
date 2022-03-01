use actix_web::{App, HttpServer};
use to_do_list_json_api_back_actix_web_postgres::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
