use actix_cors::Cors;
use actix_web::middleware::Logger;

#[macro_use]
extern crate derive_more;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let server = HttpServer::new(move || {
        let redis = RedisActor::start(&redis_config);
        App::new()
            .data(redis)
            .data(CONFIG.clone())
            .wrap(Cors::new().allowed_origin("*").send_wildcard().finish())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(all_assets)))
    })
    .bind(binding)?
    .run();
    // run server
    server.await
}
