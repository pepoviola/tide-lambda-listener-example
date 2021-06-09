#[cfg(target_env = "musl")]
use tide_lambda_listener::LambdaListener;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Pool;
use tide::prelude::*;
use tide::Server;

mod functions;
#[cfg(not(target_env = "musl"))]
use functions::get_hello;
#[cfg(not(target_env = "musl"))]
use functions::post_hello;

#[cfg(target_env = "musl")]
include!(concat!(env!("OUT_DIR"), "/lambda.rs"));

#[derive(Clone, Debug)]
pub struct State {
    db_pool: PgPool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Greeting {
    name: String,
    country_code: Option<String>, // move to enum
}

#[async_std::main]
async fn main() -> tide::http::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let db_pool = make_db_pool(&db_url).await;
    let mut app = server(db_pool).await;
    let app_ref = &mut app;

    #[cfg(target_env = "musl")]
    {
        register_route(app_ref);
        app.listen(LambdaListener::new()).await?;
    }
    #[cfg(not(target_env = "musl"))]
    {
        post_hello::register_route(app_ref);
        get_hello::register_route(app_ref);
        let mut listener = app
            .bind(format!("0.0.0.0:{}", port))
            .await
            .expect("can't bind the port");

        for info in listener.info().iter() {
            println!("Server listening on {}", info);
        }
        listener.accept().await.unwrap();
    }

    Ok(())
}

// helpers
async fn server(db_pool: PgPool) -> Server<State> {
    let state = State { db_pool };
    tide::with_state(state)
}

pub async fn make_db_pool(db_url: &str) -> PgPool {
    Pool::connect(db_url).await.unwrap()
}
