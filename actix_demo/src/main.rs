use actix_web::{middleware::Logger, App, Error, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use paperclip::actix::{
    api_v2_operation,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    get,
    post,
    // use this instead of actix_web::web
    web::{self, Json, Path, Query},
    Apiv2Schema,
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt,
};
use serde::{Deserialize, Serialize};

// This struct represents state
struct AppState {
    app_name: String,
    app_version: String,
}

#[derive(Deserialize, Apiv2Schema)]
struct QueryInfo {
    username: Option<String>,
}

#[derive(Deserialize, Apiv2Schema)]
struct BodyInfo {
    username: String,
}

#[derive(Deserialize, Apiv2Schema)]
struct PathInfo {
    user_id: uuid::Uuid,
    friend: String,    
}

#[derive(Serialize, Apiv2Schema)]
struct IPostResponse<T> {
    message: String,
    data: T,
}

#[api_v2_operation]
#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    let app_version = &data.app_version; // <- get app_name
    HttpResponse::Ok().body(format!("Hello2 to {app_name}, {app_version}!")) // <- response with app_name
}

/// extract path info from "/users/{user_id}/{friend}" url
/// extract path info using serde
#[api_v2_operation]
#[get("/users/{friend}/{user_id}")] // <- define path parameters
async fn get_user(
    info: Path<PathInfo>,
    query: Query<QueryInfo>,
) -> Result<Json<IPostResponse<String>>, Error> {
    let result = if let Some(username) = &query.username {
        format!(
            "Welcome {}, user_id {}, username {}! ",
            info.friend, info.user_id, username
        )
    } else {
        format!("Welcome {}, user_id {}! ", info.friend, info.user_id)
    };
    let response = IPostResponse {
        data: result,
        message: String::from("Data got sucessfully"),
    };
    Ok(web::Json(response))
}

/// deserialize `Info` from request's body
#[api_v2_operation]
#[post("/submit")]
async fn submit(body_info: Json<BodyInfo>) -> Result<Json<IPostResponse<String>>, Error> {
    let response = IPostResponse {
        data: format!("Welcome {}!", body_info.username),
        message: String::from("Data created sucessfully"),
    };
    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            // Record services and routes from this line.
            .wrap_api()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                app_version: String::from("v1"),
            }))
            .service(hello)
            .service(get_user)
            .service(submit)
            // Mount the v2/Swagger JSON spec at this path.
            .with_json_spec_at("/api/spec/v2")
            .with_swagger_ui_at("/docs")
            // IMPORTANT: Build the app!
            .build()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
