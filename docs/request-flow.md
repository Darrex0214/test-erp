# Flujo De Peticiones

Una petición atraviesa adaptadores externos, casos de uso y dominio. La dirección de la respuesta recorre las mismas capas en sentido contrario.

## Consulta con HTML completo

Ejemplo: el usuario visita `/inventory`.

```text
Navegador
  → GET /inventory
  → router de erp-web
  → products_page()
  → ListProductsUseCase::execute()
  → ProductRepository::list()
  → PostgresProductRepository
  → SQLx / PostgreSQL
  ← Vec<Product>
  ← conversión a Vec<ProductView>
  ← render_products()
  ← inventory/products.html
  ← HTTP 200 con HTML
```

`erp-web` no conoce la consulta SQL. `erp-infrastructure` no conoce el HTML. `erp-templates` no conoce entidades persistidas ni casos de uso.

## Comando desde un formulario

Ejemplo: el usuario publica un asiento contable.

```text
Formulario HTMX
  → POST /accounting/journal-entries
  → PostJournalEntryRequest
  → conversión de UUID, monto y moneda
  → PostJournalEntryCommand
  → PostJournalEntryUseCase::execute()
  → JournalEntry::draft()
  → Posting::new()
  → JournalEntry::post()
  → JournalEntryRepository::save()
  → PostgresAccountRepository
  → PostgreSQL
  ← resultado del caso de uso
  ← respuesta para el objetivo HTMX
```

La responsabilidad de cada paso es distinta:

- El DTO comprueba que la petición tenga la forma esperada.
- El handler convierte texto y tipos propios de HTTP.
- El caso de uso coordina la operación.
- El dominio protege invariantes y transiciones.
- El repositorio persiste el resultado.
- HTMX reemplaza la región indicada por `hx-target`.

## Renderizado de plantillas

Las funciones públicas de `erp-templates` reciben modelos serializables, seleccionan una plantilla y devuelven `Result<String, minijinja::Error>`.

```text
Entidad de dominio
  → modelo de vista en erp-web
  → contexto MiniJinja
  → plantilla HTML
  → String
  → axum::response::Html
```

La conversión a un modelo de vista evita exponer directamente la estructura interna del dominio y permite formatear estados, monedas o fechas para la interfaz.

## Errores

Los errores también viajan hacia fuera:

```text
PostgreSQL o dominio
  → DomainError
  → caso de uso
  → handler
  → estado HTTP y mensaje
  → navegador
```

Una entrada inválida debe producir un error `4xx`. Un fallo inesperado de plantilla, conexión o persistencia debe producir un error `5xx`. Al agregar handlers, no se deben convertir indiscriminadamente errores de infraestructura en `400 Bad Request`.

## Dashboard

El dashboard actual es una excepción simple porque no consulta datos:

```text
GET /
  → dashboard()
  → render_dashboard()
  → dashboard.html
  → HTTP 200
```

Si más adelante muestra métricas o módulos según permisos, los datos deben obtenerse mediante queries de `erp-application`, no desde la plantilla.
