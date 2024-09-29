use dotenv::dotenv;
use std::env;
use std::ptr::null;
use db::empleado::fetch_empleados;
use actix_web::{get,post,HttpResponse,Responder,web};
use sqlx::{MySql, MySqlPool, Pool};
use crate::db;
use crate::db::empleado::{fetch_empleado_id, save_empleado};
use crate::models::empleado::{Empleado, EmpleadoDto, FromDTO, ValidarEmp};
use env_logger::Env;
use log::{info,warn,error};
use serde::Deserialize;


#[derive(Deserialize)]
struct QueryParams{
    filtro: Option<String>,
    estado: Option<String>,
    id: Option<i32>
}

#[get("/api/empleado/get_empleados")]
async fn getempleados() ->impl Responder{
    dotenv().ok();
    info!("Se ha recibido una solicitud a get_empleados");

    let url = env::var("DATABASE_URL")
        .expect(("DATABASE_URL puede ser seteada"));
    let pool = MySqlPool::connect_lazy(&url).expect("Fallo al crear el pool");
    match fetch_empleados(&pool).await {
        Ok(empleados) => {
            info!("Empleados recuperados exitosamente");
            HttpResponse::Ok().json(empleados)
        },
        Err(_e) => {
            error!("Ha ocurrido un error en get_empleados: {:?}",_e);
            HttpResponse::BadRequest().body("ocurrio un error")
        },
    }
}
#[get("/api/empleado/get_empleado")]
async fn getempleado(params: web::Query<QueryParams>) ->impl Responder{
    dotenv().ok();
    info!("Se ha recibido una solicitud a get_empleados");
    if params.id.is_none(){
        return HttpResponse::BadRequest().body("Id no proporcionado")
    }
    let url = env::var("DATABASE_URL")
        .expect(("DATABASE_URL puede ser seteada"));
    let pool = MySqlPool::connect_lazy(&url).expect("Fallo al crear el pool");
    let _id = params.id.unwrap();
    info!("Id es: {:?}",_id);
    match fetch_empleado_id(&pool, _id).await {
        Ok(empleados) => {
            info!("Empleados recuperados exitosamente");
            HttpResponse::Ok().json(empleados)
        },
        Err(_e) => {
            error!("Ha ocurrido un error en get_empleados: {:?}",_e);
            HttpResponse::BadRequest().body("ocurrio un error")
        },
    }
}

#[post("/api/empleado/add_empleado")]
async fn addempleado(empleado: web::Json<EmpleadoDto>) -> impl Responder {
    dotenv().ok();
    info!("Se ha recibido una solicitud a add_empleado");
    if let Err(errors) = empleado.validate() {
        warn!("Errores de validacion:{:?}",errors);
        return HttpResponse::BadRequest().body(format!("Errores de validacion: {:?}", errors));
    }

    let _empleado: Empleado = Empleado::from_dto(empleado.into_inner());
    let url: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL puede ser seteada");
    let pool: Pool<MySql> = MySqlPool::connect_lazy(&url).expect("Fallo al crear el pool");

    match save_empleado(&pool, _empleado).await {
        Ok(_) => {
            info!("Empleado agregado con exito");
            HttpResponse::Ok().body("usuario agregado con exito")
        },
        Err(_e) => {
            error!("Ha ocurrido un error en add_empleado: {:?}",_e);
            HttpResponse::InternalServerError().body(format!("error al insertar empleado: {_e}"))
        }
    }
}


