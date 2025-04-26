mod httpreq;
mod query;

use crate::httpreq::{add_draft, catalogs, clients, imo, parts_by_cat, requests_by_orderid, requests_by_statuses, serials_by_cat, target_place, targets};
use crate::query::asyncdb::AsyncDB;
use axum::Router;
use axum::http::{HeaderValue, Method};
use axum::http::header::CONTENT_TYPE;
use axum::routing::{get, post};
use env_logger::{Builder, Target};
use log::LevelFilter;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: AsyncDB,
   
}
#[tokio::main]
async fn main() {
    let mut builder: Builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter(None, LevelFilter::Warn).init();
    let db = AsyncDB::new().await;
    let state = AppState { db };

    let app = Router::new()
        .route("/targets", get(targets))
        .route("/clients", get(clients))
        .route("/targets/imos/{imo}", get(imo))
        .route("/targets/catalogs/{target}", get(catalogs))
        .route("/targets/serials/{a}/{b}/{c}", get(serials_by_cat))
        .route("/targets/pbc/{a}/{b}", get(parts_by_cat))
        .route("/targets/place/{a}/{b}", get(target_place))

        .route("/adddraft", post(add_draft))
        .route("/reqbystatus/{a}", get(requests_by_statuses))
        .route("/reqbyorderid/{a}", get(requests_by_orderid))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
