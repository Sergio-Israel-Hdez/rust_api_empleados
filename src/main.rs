mod empleado;
mod db;
mod models;
use actix_web::{App,HttpServer};
use actix_web::http::header;
use actix_cors::Cors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET","POST"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3000);
        App::new()
            .wrap(cors)
            .service(empleado::getempleados)
            .service(empleado::addempleado)
            .service(empleado::getempleado)
            .service(empleado::hola)
    })
        .bind(("0.0.0.0",9001))?
        .run()
        .await
}
