use std::collections::HashMap;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use crate::models::blog::Blog;
use crate::models::response_message::ResponseMessage;
use crate::models::user::User;

#[post("/add_blog")]
pub async fn add_blog(info: web::Json<Blog>, db: web::Data<Database>) -> impl Responder {
    let blog_collection = db.collection("blogs");

    let result = blog_collection.insert_one(info.into_inner()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ResponseMessage {
            message: "Blog added successfully".into(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Error adding blog".into(),
        }),
    }
}

#[get("/user_by_email")]
pub async fn get_user_by_email(query: web::Query<HashMap<String, String>>, db: web::Data<Database>) -> impl Responder {
    let email = match query.get("email") {
        Some(email) => email,
        None => return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Email parameter is missing".into(),
        }),
    };

    let user_collection: Collection<User> = db.collection("users");
    let filter = doc! { "email": email };

    match user_collection.find_one(filter).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ResponseMessage {
            message: "User not found".into(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Error retrieving user".into(),
        }),
    }
}

#[get("/blogs")]
pub async fn get_blogs(db: web::Data<Database>) -> impl Responder {
    let blog_collection: Collection<Blog> = db.collection("blogs");
    let filter = doc! {};

    let mut cursor = match blog_collection.find(filter).await {
        Ok(cursor) => cursor,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Error getting blogs".into(),
            })
        }
    };

    let mut blogs = Vec::new();
    while let Some(blog) = cursor.next().await {
        match blog {
            Ok(blog) => blogs.push(blog),
            Err(_) => {
                return HttpResponse::InternalServerError().json(ResponseMessage {
                    message: "Error parsing blog".into(),
                })
            }
        }
    }

    HttpResponse::Ok().json(blogs)
}
