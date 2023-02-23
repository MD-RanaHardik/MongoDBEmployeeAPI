#![allow(dead_code)]
#![allow(unused_variables)]

use actix_web::{web, App, HttpResponse, HttpServer};
use mongodb::{bson::Document, Client, Collection};
use views::deleteuser;

mod api_views;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Faild to connect with server");
    let col: Collection<Document> = client.database("FirstDB").collection("Employee");
    let apicol: Collection<Document> = client.database("FirstDB").collection("APIUsers");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new((col.clone(), apicol.clone())))
            .service(views::greet)
            .service(views::getUsers)
            .service(views::getPerticulerUsersData)
            .service(views::deleteuser)
            .service(views::updateuser)
            .service(views::insertuser)
            .service(api_views::api_newuser)
            .service(api_views::generate_newtoken)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
