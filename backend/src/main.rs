use actix_web::{HttpServer, App, http};
use actix_cors::Cors;
use listenfd;
use dotenv::dotenv;
use std::env;


mod utils;
mod api;
mod schema;
mod models;
mod database;
mod python_hook;
mod mailer;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    // Initialise the logger.
    env::set_var("RUST_LOG", "actix_web = info");
    env_logger::init();
    // Get (static) variables from .env file.
    dotenv().ok();
    
    //python_hook::run_py_hook("[1, 2]".to_string()).await.expect("Error running Python hook!");
    //mailer::send_email("Hello, World!".to_string(), "hello@shayandoust.me".to_string());
    
    let mut listenfd = listenfd::ListenFd::from_env();
    let mut actix_server = HttpServer::new(|| {
        // let cors = Cors::default()
        //     .allowed_origin("https://shayandoust.github.io")
        //     .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(api::ping)
            .service(api::login_probe)
            .service(api::my_user_id)
            .service(api::new_submission)
            .service(api::top_5_submissions)
    });

    actix_server = match listenfd.take_tcp_listener(0)?
    {
        Some(listener) => actix_server.listen(listener)?,
        None => {
            let host = env::var("BIND_IP").unwrap();
            let port = env::var("BIND_PORT").unwrap();

            actix_server.bind(format!("{}:{}", host, port))?
        }
    };

    actix_server.run().await
}
