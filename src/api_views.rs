use std::{collections::HashMap, fmt::format};

use actix_web::{
    get,
    web::{self, Data, Query},
    HttpRequest, HttpResponse, Responder,
};

use mongodb::{
    bson::{doc, Document},
    options::Collation,
    Client, Collection,
};
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};

#[get("api/newuser/{username}/{password}")]
pub async fn api_newuser(
    data: web::Path<(String, String)>,
    col: Data<(Collection<Document>, Collection<Document>)>,
) -> impl Responder {
    match col.1.count_documents(doc! {"Username":&data.0}, None).await {
        Ok(v) => {
            if v > 0 {
                HttpResponse::Ok().body(format!("Usersname already exist {}", v))
            } else {
                let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
                match col.1.insert_one(doc!{"Username":&data.0,"Password":&data.1,"API_Token":&token,"Number_of_request":0}, None).await{
                    Ok(v)=>{
                        HttpResponse::Ok().body(format!("Account successfully created\nUsername : {}\nPassword : {}\nAPI_Token : {}",&data.0,&data.1,token))
                    }
                    ,Err(e)=>{
                        HttpResponse::InternalServerError().body("Faild to create new account")
                    }
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body("Faild to create new account"),
    }
}

#[get("/api/generatenewtoekn/{username}/{password}")]
pub async fn generate_newtoken(
    path: web::Path<(String, String)>,
    col: Data<(Collection<Document>, Collection<Document>)>,
) -> impl Responder {
    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    match col
        .1
        .update_one(
            doc! {"Username":&path.0,"Password":&path.1},
            doc! {"$set":{"API_Token":&token},"$inc":{"Number_of_request":1}},
            None,
        )
        .await
    {
        Ok(docu) => HttpResponse::Ok().body(format!("New Token : {}", token)),
        Err(e) => HttpResponse::InternalServerError().body("Please check username and password"),
    }
}

pub async fn check_token_and_update_number_of_request(
    api_token: String,
    col: &Collection<Document>,
) -> bool {
    match col
        .update_one(
            doc! {"API_Token":api_token.to_string()},
            doc! {"$inc":{"Number_of_request":1}},
            None,
        )
        .await
    {
        Ok(docm) => true,
        Err(e) => false,
    }
}
