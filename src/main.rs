mod empleado;
mod db;
mod models;
use actix_web::{App,HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(empleado::getempleados)
            .service(empleado::addempleado)
            .service(empleado::getempleado)
    })
        .bind(("0.0.0.0",9001))?
        .run()
        .await
}
