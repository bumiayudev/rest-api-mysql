mod handler;
mod model;
mod schema;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use dotenv::dotenv;

pub struct AppState {
    db: MySqlPool,
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new() 
            .max_connections(10)
            .connect(&database_url)
            .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        },
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }   
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState { db : pool.clone()}))
        .wrap(Logger::default())
        .service(handler::get_post)
        .service(handler::create_post)
        .service(handler::get_single_post)
        .service(handler::edit_post)
        .service(handler::delete_post)
       
    })
    .bind(("127.0.0.1", 5050))?
            .run()
            .await

}
