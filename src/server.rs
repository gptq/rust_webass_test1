use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde_json::json;
use chrono::Local;

#[get("/current_time")]
async fn current_time() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "time": Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8081");
    
    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .service(current_time)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
