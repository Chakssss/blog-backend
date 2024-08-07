use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};

//IMPORTS FROM MODELS
use crate::models::user::User;
use crate::models::login_info::LoginInfo;
use crate::models::response_message::ResponseMessage;



#[post("/")]
pub async fn login(info: web::Json<LoginInfo>, db: web::Data<Database>) -> impl Responder {
    let user_collection = db.collection("users");

    let filter = doc! { "email": &info.email };

    let user: Option<User> = user_collection
        .find_one(filter)
        .await
        .expect("Error finding user");

    match user {
        Some(user) => {
            if verify(&info.password, &user.password).expect("Error verifying password") {
                HttpResponse::Ok().json(ResponseMessage {
                    message: "Login successful".into(),
                })
            } else {
                HttpResponse::Unauthorized().json(ResponseMessage {
                    message: "Invalid credentials".into(),
                })
            }
        }
        None => HttpResponse::Unauthorized().json(ResponseMessage {
            message: "Invalid credentials".into(),
        }),
    }
}



#[post("/signup")]
pub async fn signup(user: web::Json<User>, db: web::Data<Database>) -> impl Responder {
    let user_collection = db.collection("users");

    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Error hashing password");

    let filter = doc! { "email" : &user.email };

    let existing_user = user_collection
        .find_one(filter)
        .await
        .expect("Error checking user instance");

    if existing_user.is_some() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "User already exists".into(),
        });
    }

    let new_user = User {
        name: user.name.clone(),
        email: user.email.clone(),
        password: hashed_password,
    };

    let result = user_collection.insert_one(new_user).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(ResponseMessage {
            message: "User created successfully".into(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Error inserting user".into(),
        }),
    }
}

