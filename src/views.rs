use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, Form}, HttpResponse, Responder,
};

use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::StreamExt;
use mongodb::{
    bson::{doc, Document}, Collection,
};
use serde::{Deserialize, Serialize};

use crate::api_views;

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
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../index.html"))
}

// get data of perticular employee from therir username
#[get("/users/{username}")]
pub async fn getPerticulerUsersData(
    username: web::Path<String>,
    credential: BearerAuth,
    col: Data<(Collection<Document>, Collection<Document>)>,
) -> impl Responder {
    if api_views::check_token_and_update_number_of_request(credential.token().to_string(), &col.1)
        .await
    {
        match col
            .0
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
#[get("/users")]
pub async fn getUsers(
    credential: BearerAuth,
    col: Data<(Collection<Document>, Collection<Document>)>,
) -> impl Responder {
    if api_views::check_token_and_update_number_of_request(credential.token().to_string(), &col.1)
        .await
    {
        let mut result = col.0.find(doc! {}, None).await.expect("faold");
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

#[get("/insertuser")]
pub async fn insertuser(
    credential: BearerAuth,
    data: Form<EmployeeDataInsert>,
    col: Data<(Collection<Document>, Collection<Document>)>,
) -> impl Responder {
    if api_views::check_token_and_update_number_of_request(credential.token().to_string(), &col.1)
        .await
    {
        let userdata = doc! {"Username":data.Username.to_string(),"Password":data.Password.to_string(),"Employee_name":&data.Employee_name,"Employee_salary":&data.Employee_salary,"Employee_designation":&data.Employee_designation,};
        match col.0.insert_one(userdata, None).await {
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
    credential: BearerAuth,
    data: Form<EmployeeDataUpdate>,
    col: Data<(Collection<Document>, Collection<Document>)>,
) -> impl Responder {
    if api_views::check_token_and_update_number_of_request(credential.token().to_string(), &col.1)
        .await
    {
        let update = doc! {"$set":{"Employee_name":&data.Employee_name,"Employee_salary":&data.Employee_salary,"Employee_designation":&data.Employee_designation,}};
        match col
            .0
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
#[get("/deleteuser/{username}")]
pub async fn deleteuser(
    username: web::Path<String>,
    credential: BearerAuth,
    col: Data<(Collection<Document>, Collection<Document>)>,
) -> impl Responder {
    if api_views::check_token_and_update_number_of_request(credential.token().to_string(), &col.1)
        .await
    {
        match col
            .0
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
