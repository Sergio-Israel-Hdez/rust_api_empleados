use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::macros::format_description;
use time::Date;


#[derive(Debug, FromRow)]
pub struct Empleado {
    pub id: Option<i32>,
    pub nombre: String,
    pub apellido: String,
    pub genero: String,
    pub fecha_nacimiento: String,
}
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct EmpleadoDto {
    pub id: Option<i32>,
    pub nombre: String,
    pub apellido: String,
    pub genero: String,
    pub fecha_nacimiento: String,
}

pub trait FromDTO<T> {
    fn from_dto(dto:T) -> Self;
}
impl FromDTO<EmpleadoDto> for Empleado{
    fn from_dto(dto: EmpleadoDto) -> Self {
        Empleado{
            id: None,
            nombre: dto.nombre,
            apellido: dto.apellido,
            genero: dto.genero,
            fecha_nacimiento: dto.fecha_nacimiento,
        }
    }
}

pub trait ValidarEmp {
    fn validate(&self) -> Result<(),Vec<String>>;
}
impl ValidarEmp for EmpleadoDto {
    fn validate(&self) -> Result<(),Vec<String>>{
        let mut errors = Vec::new();
        if self.nombre.is_empty() { errors.push("Nombre no puede estar vacio".to_string()) }
        if self.apellido.is_empty() { errors.push("Apellido no puede estar vacio".to_string()) }

        let format = format_description!("[year]-[month]-[day]");
        if Date::parse(&self.fecha_nacimiento, &format).is_err() {
            errors.push("Fecha de nacimiento inválida, debe ser YYYY-MM-DD".to_string());
        }
        if errors.is_empty() { Ok(()) }
        else { Err(errors) }
    }
}
