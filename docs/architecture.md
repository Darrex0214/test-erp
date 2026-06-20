# Arquitectura

Este proyecto organiza el ERP como un workspace de Rust con Arquitectura Hexagonal, DDD y separación CQRS entre comandos y consultas.

## Capas

### `erp-domain`

Contiene el modelo y las reglas de negocio:

- Entidades y raíces de agregado.
- Value objects.
- Eventos de dominio.
- Servicios de dominio.
- Puertos que describen las operaciones de persistencia.
- Errores compartidos del dominio.

No debe depender de Axum, SQLx, MiniJinja ni de otro crate del workspace.

### `erp-application`

Contiene los casos de uso organizados como comandos y consultas. Coordina entidades del dominio y puertos, pero no conoce HTTP, HTML ni PostgreSQL.

Un comando modifica estado, por ejemplo `AdjustStockUseCase`. Una consulta obtiene información sin expresar una operación de escritura, por ejemplo `ListProductsUseCase`.

### `erp-infrastructure`

Implementa adaptadores técnicos:

- Repositorios PostgreSQL mediante SQLx.
- Conversión entre filas SQL y entidades de dominio.
- Publicador de eventos en memoria.

Los repositorios implementan los traits definidos por `erp-domain`. Las operaciones que guardan una raíz de agregado y sus hijos deben ejecutarse dentro de una transacción.

El publicador en memoria existe, pero todavía no está conectado al bootstrap ni a los casos de uso. Los eventos generados por las entidades no producen efectos externos hasta completar esa integración.

### `erp-templates`

Define modelos de vista y renderiza HTML con MiniJinja. Las plantillas usan HTMX, Alpine.js y Tailwind CSS desde CDN.

Esta capa no consulta la base de datos y no contiene reglas de negocio. Recibe datos preparados para presentación y devuelve HTML.

### `erp-web`

Es el adaptador HTTP basado en Axum:

- Declara rutas.
- Deserializa formularios en DTOs.
- Convierte datos HTTP a commands o queries.
- Ejecuta casos de uso.
- Convierte entidades a modelos de vista.
- Renderiza plantillas y crea respuestas HTTP.

Los handlers deben ser delgados. Las validaciones de negocio pertenecen al dominio, no a esta capa.

### `erp-server`

Es el composition root y ejecutable del sistema:

- Carga variables de entorno.
- Configura logging.
- Crea el `PgPool`.
- Construye repositorios y casos de uso.
- Inyecta el estado de Axum.
- Arranca el servidor y expone el health check.

No debe contener reglas de negocio.

## Dirección de dependencias

```text
erp-server → erp-web → erp-application → erp-domain
     │           │
     │           ├→ erp-templates
     │           └→ erp-domain
     └→ erp-infrastructure → erp-domain
```

De forma exacta:

```text
erp-application    → erp-domain
erp-infrastructure → erp-domain
erp-web            → erp-application + erp-domain + erp-templates
erp-server         → erp-application + erp-infrastructure + erp-web
```

Las dependencias deben apuntar hacia el dominio. Si una necesidad técnica obliga al dominio a conocer SQL, HTTP o HTML, la responsabilidad está ubicada en la capa incorrecta.

## Reglas de diseño

- Las invariantes se protegen en constructores y métodos del dominio.
- Los casos de uso determinan el orden de las operaciones.
- Los puertos se declaran hacia dentro y sus adaptadores se implementan hacia fuera.
- Los DTO HTTP no se reutilizan como entidades de dominio.
- Los modelos de vista solo contienen información necesaria para renderizar.
- `erp-server` realiza la inyección explícita de dependencias.
- Un módulo no debe acceder directamente a las tablas de otro agregado sin una decisión de diseño explícita.
