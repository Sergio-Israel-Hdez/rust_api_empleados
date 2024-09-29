use sqlx::mysql::{MySqlPool, MySqlQueryResult, MySqlRow};
use sqlx::{Error, Row};
use time::Date;
use crate::db::empleado;
use crate::models::empleado::{Empleado, EmpleadoDto};

pub async fn fetch_empleados(pool: &MySqlPool) -> Result<Vec<EmpleadoDto>,sqlx::Error>{
    let rows = sqlx::query(
        "SELECT id, nombre, apellido, genero, fecha_nacimiento FROM empleados"
    )
        .fetch_all(pool)
        .await?;
    let empleados:Vec<EmpleadoDto> = rows.into_iter().map(|row|{
        let fecha_nacimiento: Date = row.get("fecha_nacimiento");
        EmpleadoDto{
            id:Some( row.get("id")),
            nombre: row.get("nombre"),
            apellido: row.get("apellido"),
            genero: row.get("genero"),
            fecha_nacimiento: fecha_nacimiento.to_string(),
        }
    }).collect();
    Ok(empleados)
}
pub async fn fetch_empleado_id(pool: &MySqlPool, id:i32) -> Result<EmpleadoDto,sqlx::Error>{
    let row = sqlx::query(
        "SELECT id, nombre, apellido, genero, fecha_nacimiento FROM empleados WHERE id = ?"
    )
        .bind(id)
        .fetch_one(pool)
        .await?;
    if row.is_empty() { return Err(Error::RowNotFound) }
    let fecha_nacimiento: Date = row.get("fecha_nacimiento");
    let empleado = EmpleadoDto{
        id:Some( row.get("id")),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        genero: row.get("genero"),
        fecha_nacimiento: fecha_nacimiento.to_string(),
    };
    Ok(empleado)
}
pub async fn save_empleado(pool: &MySqlPool,empleado: Empleado) -> Result<(),sqlx::Error>{
    let _:Result<MySqlQueryResult,Error> = sqlx::query(
        "INSERT INTO empleados (nombre, apellido, genero, fecha_nacimiento) VALUES(?,?,?,?)")
        .bind(&empleado.nombre)
        .bind(&empleado.apellido)
        .bind(&empleado.genero)
        .bind(&empleado.fecha_nacimiento)
        .execute(pool).await;
    Ok(())
}