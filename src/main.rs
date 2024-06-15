use config::Config;
use http::{
    blog, 
    //cv_english, 
    //cv_swedish, 
    files, 
    index, 
    projects
};
use ntex::web::{get, middleware, App, HttpServer};

mod config;
mod http;

/*
 * Main function, the base of the entire website as a whole
 * Manages all site structure and initializes state on startup.
 */

#[ntex::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::get();
    let ip = config.ip();
    let port = config.port();
    let custom_headers =
        if config.custom_header_key().is_some() && config.custom_header_value().is_some() {
            (
                config.custom_header_key().expect("Can't get header key"),
                config
                    .custom_header_value()
                    .expect("Can't get header value"),
            )
        } else {
            ("Empty-Header".to_string(), "Ignore".to_string())
        };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::default()
                    .header(custom_headers.0.clone(), custom_headers.1.clone()),
            )
            .service(index)
            .service(blog)
            .service(projects)
            //.service(cv_english)
            //.service(cv_swedish)
            .route("/{filename}*", get().to(files))
            .wrap(ntex::web::middleware::Logger::default())
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
