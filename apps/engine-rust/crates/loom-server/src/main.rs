use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("LOOM_SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(7700);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("loom-server: cannot bind {addr}: {e}");
            std::process::exit(1);
        }
    };
    println!("loom-server listening on {addr}");

    if let Err(e) = axum::serve(listener, loom_server::app())
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        eprintln!("loom-server: {e}");
        std::process::exit(1);
    }
}

async fn shutdown_signal() {
    // Ignore installation failure only in the impossible case the handler
    // can't be registered; then we simply never shut down gracefully.
    if tokio::signal::ctrl_c().await.is_err() {
        std::future::pending::<()>().await;
    }
}
