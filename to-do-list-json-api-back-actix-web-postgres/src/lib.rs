#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn it_works() {
        let mut app = test::init_service(
            App::new()
                .wrap(middleware::Logger::default())
                .service(index),
        )
        .await;
    }
}
