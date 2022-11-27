use actix_web::{web, App, HttpRequest, HttpServer, Responder};
mod state;
mod to_do;
mod views;
mod processes;


async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello: {} ", name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        println!("Function is firing.");
        let app = App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet));
        
        return app
        })
        .bind("127.0.0.1:8000")?
        .workers(3)
        .run()
        .await
}