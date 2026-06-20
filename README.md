# Rust ERP

ERP modular en Rust con Arquitectura Hexagonal, DDD y CQRS. El dominio queda en el centro y no conoce Axum, SQLx ni detalles de infraestructura.

## Arquitectura

- `erp-domain`: entidades, value objects, eventos, puertos y reglas de negocio.
- `erp-application`: casos de uso CQRS con inyección explícita de dependencias.
- `erp-infrastructure`: adapters concretos para PostgreSQL con SQLx y publicación de eventos en memoria.
- `erp-web`: handlers delgados de Axum y router HTTP.
- `erp-templates`: plantillas MiniJinja con HTMX, Alpine.js y Tailwind CSS por CDN.
- `erp-server`: bootstrap, configuración, PgPool, DI manual y arranque del servidor.

Las dependencias apuntan hacia dentro: presentación e infraestructura dependen de aplicación/dominio, pero el dominio no depende de frameworks.

## Requisitos

- Rust `1.96.0`
- PostgreSQL `18.4`
- Base de datos configurada en `DATABASE_URL`

## Ejecución

```bash
cp .env.example .env
sqlx database create
sqlx migrate run
cargo run -p erp-server
```

El servidor escucha en `127.0.0.1:3000`.

## Validación

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

## Documentación técnica

- [Stack y versiones](docs/stack.md)
- [Arquitectura](docs/architecture.md)
- [Guía para crear módulos](docs/module-guide.md)
- [Flujo de peticiones](docs/request-flow.md)
- [Base de datos](docs/database.md)
- [Entorno de desarrollo](docs/development.md)

## Módulos iniciales

- Contabilidad: cuentas, asientos contables, partida doble y eventos de asiento publicado.
- Inventario: productos, SKU, stock y eventos de cambio de inventario.
- Facturación: clientes, facturas, líneas, emisión, pago y cancelación con reglas de dominio.
