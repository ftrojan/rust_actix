use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use serde::{Serialize, Deserialize};

struct AppState {
    foo: String,
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    name: String,
}

#[get("/json")]
async fn json() -> impl Responder {
    let x = MyObj { name: "user" };
    let body = serde_json::to_string(&x).unwrap();
    HttpResponse::Ok().content_type("application/json").body(body)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/form")]
async fn form() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../static/form.html"))
}

#[post("/post1")]
async fn handle_post_1(params: web::Form<MyParams>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}

/// State and POST Params
#[post("/post2")]
async fn handle_post_2(
    state: web::Data<AppState>,
    params: web::Form<MyParams>,
) -> HttpResponse {
    HttpResponse::Ok().content_type("text/plain").body(format!(
        "Your name is {}, and in AppState I have foo: {}",
        params.name, state.foo
    ))
}

/// Request and POST Params
#[post("/post3")]
async fn handle_post_3(req: HttpRequest, params: web::Form<MyParams>) -> impl Responder {
    println!("Handling POST request: {:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name))
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .data(AppState {foo: "bar".to_string()})
            .service(hello)
            .service(form)
            .service(echo)
            .service(json)
            .service(handle_post_1)
            .service(handle_post_2)
            .service(handle_post_3)
            .route("/hey", web::get().to(manual_hello))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(app_config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
