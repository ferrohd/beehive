use std::net::SocketAddr;

use axum::Router;
use bb8::Pool;
use bb8_postgres::{tokio_postgres,PostgresConnectionManager};
use route::routes;
use tokio::signal;
use tower_http::trace::TraceLayer;

pub mod error;
pub mod model;
mod route;

pub type Postgres = Pool<PostgresConnectionManager<tokio_postgres::NoTls>>;

#[derive(Clone)]
pub struct Config {
    pub min_coupon_create_reputation: i32,
    pub create_coupon_cost: i32,
    pub vote_coupon_cost: i32,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            min_coupon_create_reputation: 10,
            create_coupon_cost: 5,
            vote_coupon_cost: 1,
        }
    }
}
#[derive(Clone)]
pub struct AppState {
    pub db_client: Postgres,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=postgres password=postgres dbname=postgres",
        tokio_postgres::NoTls,
    )?;
    let pool = Pool::builder().build(manager).await?;

    let state = AppState {
        db_client: pool,
        config: Config::default(),
    };

    let app = Router::new()
        .nest("/", routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
