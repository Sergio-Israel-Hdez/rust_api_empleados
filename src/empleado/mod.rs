use dotenv::dotenv;
use std::env;
use std::ptr::null;
use db::empleado::fetch_empleados;
use actix_web::{get,post,HttpResponse,Responder,web};
use sqlx::{MySql, MySqlPool, Pool};
use crate::db;
use crate::db::empleado::save_empleado;
use crate::models::empleado::{Empleado, EmpleadoDto, FromDTO, ValidarEmp};
use env_logger::Env;
use log::{info,warn,error};


#[get("/api/get_empleados")]
async fn getempleados() ->impl Responder{
    dotenv().ok();
    info!("se a recibido una solicitud a get_empleados");

    let url = env::var("DATABASE_URL")
        .expect(("DATABASE_URL puede ser seteada"));
    let pool = MySqlPool::connect_lazy(&url).expect("Fallo al crear el pool");
    match fetch_empleados(&pool).await {
        Ok(empleados) => HttpResponse::Ok().json(empleados),
        Err(_e) => {
            error!("a ocurrido un error en get_empleados,URL BD{url}");
            HttpResponse::BadRequest().body("ocurrio un error")
        },
    }
}


#[post("/api/add_empleado")]
async fn addempleado(empleado: web::Json<EmpleadoDto>) -> impl Responder {
    dotenv().ok();

    if let Err(errors) = empleado.validate() {
        return HttpResponse::BadRequest().body(format!("Errores de validacion: {:?}", errors));
    }

    let _empleado: Empleado = Empleado::from_dto(empleado.into_inner());
    let url: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL puede ser seteada");
    let pool: Pool<MySql> = MySqlPool::connect_lazy(&url).expect("Fallo al crear el pool");

    match save_empleado(&pool, _empleado).await {
        Ok(_) => HttpResponse::Ok().body("usuario agregado con exito"),
        Err(e) => HttpResponse::InternalServerError().body(format!("error al insertar empleado: {e}")),
    }
}


