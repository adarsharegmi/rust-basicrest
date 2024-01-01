use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, 
    Router,
    extract::{Path, Query}
};
use serde::{Serialize, Deserialize};


#[derive(Serialize)]
struct User {
    name: String,
    age: i32,
    email: String,
}



// handler for creating a user

async fn create_user() -> impl IntoResponse{
    Response::builder()
    .status(StatusCode::CREATED)
    .body(Body::from("user created successfully"))
    .unwrap()
}


async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            name: "Addy".to_string(),
            age: 24,
            email: "adarsharegmi121@gmail.com".to_string()
        },
        User {
            name: "shaurav".to_string(),
            age:30,
            email:"palpalishaurav@gmail.com".to_string()
        }
    ];
    Json(users)
}


// Extractors


#[derive(Deserialize)]
struct Page {
    number: u32,
}


// handler for path and query
async fn show_item(Path(id): Path<u32>,  Query(page) : Query<Page>) -> String{
    format!("Item {} on page {}", id, page.number)
}




//post method request  body 

#[derive(Deserialize)]
struct Item {
    title: String,
    name: String,
}

// handler for getting data from request body

async fn create_data(Json(data): Json<Item>) -> String {
    format!("Item  title is {}", data.title)
}


#[tokio::main]
async fn main() {
    let ip_address = "0.0.0.0:8000";
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users))
        .route("/item/:id", get(show_item))
        .route("/add-item", post(create_data));

    println!("Running on http://localhost:8000");

    axum::Server::bind(&ip_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}