# Guía Para Crear Un Módulo

Esta guía usa un módulo hipotético `work_orders` para mostrar cómo extender el ERP sin romper sus límites arquitectónicos.

## 1. Modelar el dominio

Crear la estructura en `crates/erp-domain/src/work_orders/`:

```text
work_orders/
├── entities/
│   ├── mod.rs
│   └── work_order.rs
├── events/
│   └── mod.rs
├── ports/
│   ├── mod.rs
│   └── repository.rs
├── value_objects/
│   └── mod.rs
└── mod.rs
```

La entidad `WorkOrder` debe proteger reglas como título obligatorio, estados válidos y transiciones permitidas. El puerto `WorkOrderRepository` expresa lo que necesita el dominio sin mencionar SQLx.

Registrar el módulo en `erp-domain/src/lib.rs`:

```rust
pub mod work_orders;
```

## 2. Crear casos de uso

Crear la estructura en `crates/erp-application/src/work_orders/`:

```text
work_orders/
├── commands/
│   ├── create_work_order.rs
│   └── mod.rs
├── queries/
│   ├── list_work_orders.rs
│   └── mod.rs
└── mod.rs
```

El command recibe datos independientes de HTTP, crea la entidad mediante el dominio y la guarda usando `Arc<dyn WorkOrderRepository>`. La query usa el mismo puerto para recuperar órdenes.

Registrar el módulo en `erp-application/src/lib.rs`.

## 3. Crear la migración y el adaptador

Crear una migración nueva, sin modificar una migración ya aplicada:

```bash
sqlx migrate add create_work_orders
```

Agregar el adaptador en:

```text
crates/erp-infrastructure/src/persistence/postgres/work_order_repository.rs
```

El adaptador implementa `WorkOrderRepository`, usa tipos SQL internos para las filas y reconstruye entidades mediante métodos `rehydrate`. Las escrituras de un agregado completo deben ser transaccionales.

## 4. Definir DTOs y handlers HTTP

Agregar `CreateWorkOrderRequest` en `crates/erp-web/src/dto/requests.rs` y crear:

```text
crates/erp-web/src/handlers/work_orders.rs
```

El handler convierte el formulario a `CreateWorkOrderCommand`, ejecuta el caso de uso y traduce el resultado a una respuesta HTTP. No debe construir SQL ni duplicar validaciones del dominio.

Registrar las rutas en `crates/erp-web/src/router.rs`:

```text
GET  /work-orders
POST /work-orders
```

Agregar los casos de uso al `AppState`.

## 5. Crear la presentación

Crear las plantillas:

```text
crates/erp-templates/templates/work-orders/
├── index.html
└── work-order-list.html
```

Definir `WorkOrderView` y sus funciones de renderizado en `crates/erp-templates/src/lib.rs`. Registrar cada plantilla en el entorno MiniJinja porque actualmente se incluyen en el binario mediante `include_str!`.

Agregar una tarjeta en `templates/dashboard.html` que enlace a `/work-orders`.

## 6. Conectar el bootstrap

En `crates/erp-server/src/main.rs`:

1. Crear `PostgresWorkOrderRepository` usando el `PgPool`.
2. Construir `CreateWorkOrderUseCase` y `ListWorkOrdersUseCase`.
3. Añadirlos al `AppState`.

Este es el único lugar que debe conocer simultáneamente implementaciones concretas y abstracciones de casos de uso.

## Comprobación final

Antes de considerar terminado el módulo, verificar que:

- El dominio no importa crates de infraestructura o presentación.
- Las reglas de negocio están en el dominio.
- Los handlers solo traducen y coordinan HTTP.
- Las escrituras compuestas usan una transacción.
- La migración funciona en una base de datos vacía.
- El módulo está enlazado desde el dashboard cuando debe ser visible.
- Formato, compilación y validaciones del workspace pasan correctamente.
