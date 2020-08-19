mod actor;
mod handler;
mod source;

use crate::handler::get_banner;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use source::load_csv;
use std::env;
use std::env::var_os;
use tera::Tera;

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate log;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let server = HttpServer::new(move || {
        let path = var_os("SOURCE_PATH")
            .ok_or("Environment 'SOURCE_PATH' not found")
            .unwrap();
        let csv_config = load_csv(path);
        csv_config.iter().for_each(|(category, counter)| {
            dbg!(category);
            dbg!(counter);
        });
        let tera_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*");
        dbg!(tera_path);
        let tera = Tera::new(tera_path).unwrap();
        App::new()
            // .data(csv_config)
            .data(tera)
            .wrap(Cors::new().allowed_origin("*").send_wildcard().finish())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(get_banner)))
    })
    .bind("0.0.0.0:8686")?
    .run();
    // run server
    server.await
}
