use axum::routing::get;
use sqlx::postgres::PgPoolOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn health_endpoint_returns_ok_when_database_is_available() {
    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        return;
    };

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("DATABASE_URL debe apuntar a PostgreSQL disponible");

    let health_pool = pool.clone();
    let app = axum::Router::new().route(
        "/health",
        get(move || erp_server::health(health_pool.clone())),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("puerto de prueba");
    let addr = listener.local_addr().expect("dirección local");
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("servidor de prueba");
    });

    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("conectar al servidor");
    stream
        .write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .expect("enviar request");

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .await
        .expect("leer response");

    server.abort();
    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
}
