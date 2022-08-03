use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tracing;
use tracing_subscriber;

mod routes;
mod types;
mod utils;
use crate::routes::default;
use crate::routes::admin;

static IP: &str = "localhost";
static PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        // .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let tera = {
        let mut tera = match tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql", ".htm", ".xml"]);
        Arc::new(tera)
    };
    tracing::info!("Started http server: {}:{}", IP, PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(actix_files::Files::new("/assets", "./assets"))
            .service(admin::admin_index)
            .service(admin::admin_page)
            .service(default::yes)
            .service(default::index)
            .service(default::selected_page)
        // .service(admin)
    })
    .bind((IP, PORT))?
    .run()
    .await
    .unwrap();
    Ok(())
}
