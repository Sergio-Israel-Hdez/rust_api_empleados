# Etapa 1: Construcción del binario
FROM rust:1.78-slim-buster AS builder

# Instalar las dependencias necesarias para compilar con OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Establecer el directorio de trabajo
WORKDIR /app


# Copiar el contenido de tu proyecto
COPY . .

# Compilar el proyecto en modo release
RUN cargo build --release

# Etapa 2: Imagen mínima para ejecutar el binario
FROM debian:buster-slim

# Instalar las dependencias necesarias para ejecutar el binario
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/* \

# Establecer el directorio de trabajo
WORKDIR /usr/local/bin

# Copiar el binario compilado desde la etapa de construcción
COPY --from=builder /app/target/release/api_empleados .

# Definir el comando de inicio
CMD ["./api_empleados"]