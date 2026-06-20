# Entorno De Desarrollo

## Requisitos

- Rust y Cargo `1.96.0`.
- PostgreSQL `18.4`.
- `sqlx-cli` compatible con SQLx `0.9.0`.

Las demás versiones están registradas en [stack.md](stack.md).

## Preparación

Crear la configuración local:

```bash
cp .env.example .env
```

Instalar la CLI si `sqlx --version` no está disponible:

```bash
cargo install sqlx-cli --version 0.9.0 --no-default-features --features rustls,postgres
```

Crear la base y aplicar las migraciones:

```bash
sqlx database create
sqlx migrate run
```

El archivo `.env` contiene configuración local y está ignorado por Git. `.env.example` debe mantenerse actualizado sin incluir secretos reales.

## PostgreSQL con Docker

Como alternativa a una instalación local:

```bash
docker run --name test-erp-postgres \
  -e POSTGRES_USER=erp \
  -e POSTGRES_PASSWORD=erp \
  -e POSTGRES_DB=erp \
  -p 5432:5432 \
  -d postgres:18.4
```

La URL correspondiente es:

```dotenv
DATABASE_URL=postgres://erp:erp@localhost:5432/erp
```

En ejecuciones posteriores se puede reutilizar con:

```bash
docker start test-erp-postgres
```

## Ejecución

```bash
cargo run -p erp-server
```

La aplicación queda disponible en:

```text
http://127.0.0.1:3000
```

El endpoint `GET /health` comprueba la conexión a PostgreSQL y responde `200 OK` cuando está disponible.

## Validación local

Antes de integrar cambios:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

El workspace raíz es un manifest virtual. Los archivos colocados directamente en `tests/` bajo la raíz no son descubiertos por Cargo; las pruebas de integración deben pertenecer a un crate miembro del workspace.

## Logging

El nivel de logging se controla con `RUST_LOG`. El valor de ejemplo es:

```dotenv
RUST_LOG=erp_server=info,erp_infrastructure=info,tower_http=info
```

El bootstrap carga esta variable mediante `tracing-subscriber`.
