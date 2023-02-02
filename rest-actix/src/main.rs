use actix_multipart::Multipart;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono;
use env_logger::Env;
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Deserialize)]
struct ReqObj {
    name: String,
    number: i32,
}

#[derive(Debug, Serialize)]
struct ResObj {
    name: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form action="/upload" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[post("/upload")]
async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let path = "./upload";
    web::block(move || std::fs::create_dir_all(path)).await??;

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let file = std::path::Path::new(content_disposition.get_filename().unwrap());
        let filenames = [file.file_stem(), file.extension()]
            .map(|e| e.map(|s| s.to_str()).flatten().unwrap())
            .join(".");
        let dt = chrono::Local::now().format("%y%m%d%f");
        let filepath = format!("./upload/{dt}_{filenames}");
        println!("{}", filepath);
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }
    Ok(HttpResponse::Ok().body("upload completed"))
}
#[get("")]
async fn hello() -> impl Responder {
    "hello world"
}

#[post("/user")]
async fn post_json(item: web::Json<ReqObj>, req: HttpRequest) -> impl Responder {
    println!("request: {req:?}");
    println!("model: {item:?}");
    let res_obj = ResObj {
        name: "user".to_string(),
    };
    web::Json(res_obj)
    // HttpResponse::Ok().json(web::Json(res_obj))
}

async fn manual_hello() -> impl Responder {
    "Hey there!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default())
            .service(index)
            .service(save_file)
            .service(web::scope("/hello").service(hello))
            .service(web::scope("/json").service(post_json))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
