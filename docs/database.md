# Base De Datos

El adaptador de persistencia usa PostgreSQL y SQLx. La conexión se configura mediante `DATABASE_URL`.

## Configuración

Ejemplo local:

```dotenv
DATABASE_URL=postgres://erp:erp@localhost:5432/erp
```

La aplicación crea un `PgPool` durante el arranque. Si la conexión falla, el servidor no comienza a escuchar peticiones.

## Migraciones

Las migraciones viven en `migrations/` y se ejecutan en orden por su prefijo numérico.

```bash
sqlx database create
sqlx migrate run
```

Para crear un cambio:

```bash
sqlx migrate add nombre_del_cambio
```

No se debe editar una migración que ya haya sido aplicada en entornos compartidos. Se crea una migración adicional que lleve el esquema del estado anterior al nuevo.

## Esquema actual

La migración inicial crea:

- `journal_entries` y `journal_postings`.
- `products`.
- `invoices` e `invoice_lines`.

`journal_postings` e `invoice_lines` pertenecen a sus respectivas raíces y usan `ON DELETE CASCADE`.

El dominio incluye entidades `Account` y `Customer`, pero el esquema actual todavía no incluye tablas `accounts` ni `customers`. Por ello, `account_id` y `customer_id` no tienen claves foráneas. Esta diferencia debe resolverse antes de considerar completos esos módulos.

## Repositorios

Los traits de repositorio se declaran en `erp-domain`. Sus implementaciones PostgreSQL viven en:

```text
crates/erp-infrastructure/src/persistence/postgres/
```

Cada adaptador debe:

- Convertir errores SQLx al error compartido definido por el dominio.
- Mapear filas mediante estructuras privadas.
- Rehidratar entidades sin generar eventos nuevos.
- Mantener las invariantes al convertir tipos persistidos.
- Evitar que SQL o tipos de SQLx escapen hacia el dominio.

## Transacciones

Guardar una raíz y sus colecciones debe ser una única operación atómica. Por ejemplo, una factura y sus líneas deben usar la misma transacción:

```text
BEGIN
  insertar o actualizar factura
  eliminar líneas anteriores cuando corresponda
  insertar todas las líneas
COMMIT
```

Ante cualquier error se debe ejecutar `ROLLBACK`. Los repositorios actuales de facturas y asientos realizan varias consultas sin transacción; es una deuda técnica que debe corregirse antes de usar el proyecto con datos importantes.

## Restricciones

Las reglas centrales deben existir en el dominio y, cuando sea posible, reforzarse en PostgreSQL con `NOT NULL`, `UNIQUE`, `CHECK` y claves foráneas. Las restricciones de base de datos no reemplazan las reglas de dominio: protegen los datos ante errores de integración o escrituras externas.
