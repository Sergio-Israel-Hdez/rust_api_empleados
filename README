# MySQL - Rust Service Consumption Project

This project consumes an existing MySQL service. Follow the steps below to configure and run the project.

## Configuration Steps

### 1. Create a Docker Network

First, create a Docker network called `my-network-mysql`:

```bash
docker network create my-network-mysql
```

### 1.1 Modify the Environment Variable

Modify the `DATABASE_URL` environment variable with the required values for your MySQL connection. Make sure to replace the values between `<>` with the correct information:

```
DATABASE_URL=mysql://<user>:<password>@<host>:<port>/<database_name>
```

### 2. Run Docker Compose

Once the environment variable is configured, run the following command to start the solution:

```bash
docker-compose up -d
```

This command will start the services defined in your `docker-compose.yml` file in detached mode.

## Create the Employees Table

After configuring and running the project, you'll need to create the `employees` table in your MySQL database. Use the following SQL script to create the table:

```sql
CREATE TABLE `employees` (
  `id` int NOT NULL AUTO_INCREMENT,
  `first_name` varchar(50) DEFAULT NULL,
  `last_name` varchar(50) DEFAULT NULL,
  `gender` enum('M','F') NOT NULL,
  `birth_date` date DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=40799 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
```

You can execute this script using a MySQL administration tool such as MySQL Workbench, phpMyAdmin, or directly from the MySQL command line.

## Additional Notes

- Make sure that the MySQL service you're connecting to is accessible from the `my-network-mysql` network.
- Verify that all necessary services are correctly defined in your `docker-compose.yml` file.
- To view the logs of the services, you can use the `docker-compose logs` command.
- After creating the `employees` table, ensure that your application has the necessary permissions to perform CRUD operations (Create, Read, Update, Delete) on this table.

If you encounter any issues during setup or execution, review the network configuration and database credentials.
# Proyecto de Consumo de Servicio MySQL - Rust

Este proyecto consume un servicio MySQL existente. Siga los pasos a continuación para configurar y ejecutar el proyecto.

## Pasos de Configuración

### 1. Crear una red Docker

Primero, cree una red Docker llamada `my-network-mysql`:

```bash
docker network create my-network-mysql
```

### 1.1 Modificar la variable de entorno

Modifique la variable de entorno `DATABASE_URL` con los valores requeridos para su conexión MySQL. Asegúrese de reemplazar los valores entre `<>` con la información correcta:

```
DATABASE_URL=mysql://<usuario>:<contraseña>@<host>:<puerto>/<nombre_base_de_datos>
```

### 2. Ejecutar Docker Compose

Una vez configurada la variable de entorno, ejecute el siguiente comando para iniciar la solución:

```bash
docker-compose up -d
```

Este comando iniciará los servicios definidos en su archivo `docker-compose.yml` en modo detached.

## Crear la tabla de empleados

Después de configurar y ejecutar el proyecto, necesitará crear la tabla `empleados` en su base de datos MySQL. Utilice el siguiente script SQL para crear la tabla:

```sql
CREATE TABLE `empleados` (
  `id` int NOT NULL AUTO_INCREMENT,
  `nombre` varchar(50) DEFAULT NULL,
  `apellido` varchar(50) DEFAULT NULL,
  `genero` enum('M','F') NOT NULL,
  `fecha_nacimiento` date DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=40799 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
```

Puede ejecutar este script utilizando una herramienta de administración de MySQL como MySQL Workbench, phpMyAdmin, o directamente desde la línea de comandos de MySQL.

## Notas Adicionales

- Asegúrese de que el servicio MySQL al que se está conectando esté accesible desde la red `my-network-mysql`.
- Verifique que todos los servicios necesarios estén definidos correctamente en su archivo `docker-compose.yml`.
- Para ver los logs de los servicios, puede usar el comando `docker-compose logs`.
- Después de crear la tabla `empleados`, asegúrese de que su aplicación tenga los permisos necesarios para realizar operaciones CRUD (Crear, Leer, Actualizar, Eliminar) en esta tabla.

Si encuentra algún problema durante la configuración o ejecución, revise la configuración de red y las credenciales de la base de datos.