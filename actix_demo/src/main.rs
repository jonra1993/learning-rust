use actix_web::{
    get, post,
    web::{self, Json},
    App, Error, HttpResponse, HttpServer, Responder, middleware::Logger,
};
use serde::{Deserialize, Serialize};
use env_logger::Env;

// This struct represents state
struct AppState {
    app_name: String,
    app_version: String,
}

#[derive(Deserialize)]
struct PathInfo {
    user_id: u32,
    friend: String,
}

#[derive(Deserialize)]
struct QueryInfo {
    username: Option<String>,
}

#[derive(Deserialize)]
struct BodyInfo {
    username: String,
}

#[derive(Serialize)]
struct IPostResponse<T> {
    message: String,
    data: T,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    let app_version = &data.app_version; // <- get app_name
    HttpResponse::Ok().body(format!("Hello2 to {app_name}, {app_version}!")) // <- response with app_name
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// extract path info from "/users/{user_id}/{friend}" url
/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn get_user(
    info: web::Path<PathInfo>,
    query: web::Query<QueryInfo>,
) -> Result<String, Error> {
    if let Some(username) = &query.username {
        Ok(format!(
            "Welcome {}, user_id {}, username {}! ",
            info.friend, info.user_id, username
        ))
    } else {
        Ok(format!(
            "Welcome {}, user_id {}! ",
            info.friend, info.user_id
        ))
    }
}

/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(body_info: web::Json<BodyInfo>) -> Result<Json<IPostResponse<String>>, Error> {
    let response = IPostResponse {
        data: format!("Welcome {}!", body_info.username),
        message: String::from("Data created sucessfully"),
    };
    Ok(web::Json(response))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                app_version: String::from("v1"),
            }))
            .service(hello)
            .service(echo)
            .service(get_user)
            .service(submit)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
