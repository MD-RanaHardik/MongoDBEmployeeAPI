use actix_web::{App, HttpServer};
use views::deleteuser;

mod views;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(views::greet)
            .service(views::getUsers)
            .service(views::getPerticulerUsersData)
            .service(views::deleteuser)
            .service(views::updateuser)
            .service(views::insertuser)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
