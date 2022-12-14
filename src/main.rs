use actix_web::{web, App, HttpRequest, HttpServer, Responder};
mod state;
mod to_do;
mod views;
mod processes;
mod json_serialization;


async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello: {} ", name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    println!("[#]Server running on 127.0.0.1:8000");

    HttpServer::new(||{
        let app = App::new().configure(
            views::views_factory);
        return app
    }).bind("127.0.0.1:8000")?.run().await
}