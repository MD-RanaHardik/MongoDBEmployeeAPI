use actix_web::{
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse, Responder, http::StatusCode,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use futures::StreamExt;
use mongodb::{
    bson::{doc, Document},
    options::Credential,
    Client, Collection,
};
use serde::{Deserialize, Serialize};



// change if you want to check username and password with diffent credential
const USERNAME: &str = "Hardik";
const PASSWORD: &str = "Hardik@@123";


// get request body and add to EmployeeDataUpdate struct
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDataUpdate {
    Employee_name: String,
    Employee_salary: u32,
    Employee_designation: String,
}

// get request body and add to EmployeeDataInsert struct
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDataInsert {
    Username: String,
    Password: String,
    Employee_name: String,
    Employee_salary: u32,
    Employee_designation: String,
}


// view for index / main page of api
#[get("/")]
pub async fn greet() -> impl Responder {
    HttpResponse::build(StatusCode::OK).content_type("text/html; charset=utf-8").body(include_str!("../index.html"))
}


// get data of perticular employee from therir username
#[post("/users/{username}")]
pub async fn getPerticulerUsersData(
    username: web::Path<String>,
    credential: BasicAuth,
) -> impl Responder {
    if credential.user_id().eq(USERNAME) && credential.password().unwrap().eq(PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("Faild to connect with server");

        let col: Collection<Document> = client.database("FirstDB").collection("Employee");

        match col
            .find(
                doc! {
                "Username":
                username.to_string()
                },
                None,
            )
            .await
        {
            Ok(mut docm) => {
                let data = &mut docm.next().await.unwrap().unwrap();
                let sdata = serde_json::to_string_pretty(&data).unwrap();
                HttpResponse::Ok().body(sdata)
            }
            Err(e) => HttpResponse::InternalServerError().body("No user found this username"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}


// get data of all employee 
#[post("/users")]
pub async fn getUsers(credential: BasicAuth) -> impl Responder {
    if credential.user_id().eq(USERNAME) && credential.password().unwrap().eq(PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap();

        let col: Collection<Document> = client.database("FirstDB").collection("Employee");
        let mut result = col.find(doc! {}, None).await.expect("faold");
        let mut data: Vec<Document> = vec![];

        while let Some(Ok(i)) = result.next().await {
            data.push(i);
        }
        let serialized_data = serde_json::to_string_pretty(&data).unwrap();

        HttpResponse::Ok().body(serialized_data)
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}

#[post("/insertuser")]
pub async fn insertuser(credential: BasicAuth, data: Json<EmployeeDataInsert>) -> impl Responder {
    if credential.user_id().eq(USERNAME) && credential.password().unwrap().eq(PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("Faild to connect with server");

        let col: Collection<Document> = client.database("FirstDB").collection("Employee");

        let userdata = doc! {"Username":data.Username.to_string(),"Password":data.Password.to_string(),"Employee_name":&data.Employee_name,"Employee_salary":&data.Employee_salary,"Employee_designation":&data.Employee_designation,};
        match col.insert_one(userdata, None).await {
            Ok(v) => HttpResponse::Ok().body("Successfully added new employee"),
            Err(e) => HttpResponse::InternalServerError().body("No user found this username"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}


// update the employee data from their username 
#[post("/updateuser/{username}")]
pub async fn updateuser(
    username: web::Path<String>,
    credential: BasicAuth,
    data: Json<EmployeeDataUpdate>,
) -> impl Responder {
    if credential.user_id().eq(USERNAME) && credential.password().unwrap().eq(PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("Faild to connect with server");

        let col: Collection<Document> = client.database("FirstDB").collection("Employee");

        let update = doc! {"$set":{"Employee_name":&data.Employee_name,"Employee_salary":&data.Employee_salary,"Employee_designation":&data.Employee_designation,}};
        match col
            .update_one(
                doc! {
                "Username":
                username.to_string()},
                update,
                None,
            )
            .await
        {
            Ok(v) => HttpResponse::Ok().body("User data successfully updated"),
            Err(e) => HttpResponse::InternalServerError().body("No user found this username"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}


// delete the employee data from their username 
#[post("/deleteuser/{username}")]
pub async fn deleteuser(username: web::Path<String>, credential: BasicAuth) -> impl Responder {
    if credential.user_id().eq(USERNAME) && credential.password().unwrap().eq(PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("Faild to connect with server");

        let col: Collection<Document> = client.database("FirstDB").collection("Employee");

        match col
            .delete_one(doc! {"Username":username.to_string()}, None)
            .await
        {
            Ok(v) => HttpResponse::Ok().body("User successfully deleted"),
            Err(e) => HttpResponse::InternalServerError().body("No user found for this username"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}
