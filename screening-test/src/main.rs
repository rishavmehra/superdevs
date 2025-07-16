use actix_web::{App, HttpServer};
use std::io;

mod routes;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let host = "0.0.0.0"; 
    let port = 8080;
    
    println!("Starting Solana HTTP server at http://{}:{}", host, port);
    
    HttpServer::new(|| {
        App::new()
            .service(routes::keypair::generate_keypair)
            .service(routes::token_create::create_token)
            .service(routes::token_mint::mint_token)
            .service(routes::message_sign::sign_message_route)
            .service(routes::message_verify::verify_message)
            .service(routes::send_sol::send_sol)
            .service(routes::send_token::send_token)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}