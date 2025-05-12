mod httpreq;
mod query;

use crate::httpreq::{accept_summ_price, add_draft, catalogs, change_status, clients, create_contract, create_post_processes, create_supp_contract, create_supp_delivery, create_supp_payments, imo, parts_by_cat, requests_by_orderid, requests_by_orderid_status, requests_by_statuses, requests_supplyers_by_orderid, serials_by_cat, target_place, targets, update_summ_price, update_supplier, update_supplier_price};
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

       
        .route("/reqbystatus/{a}", get(requests_by_statuses))
        .route("/reqbyorderid/{a}", get(requests_by_orderid))
        .route("/reqbyorderidstatus/{a}/{b}", get(requests_by_orderid_status))
        .route("/reqsuppbyorderid/{a}", get(requests_supplyers_by_orderid))

        .route("/adddraft", post(add_draft))
        .route("/changestatus", post(change_status))
        .route("/updatesupplyer", post(update_supplier))
        .route("/updatesupplyerprice", post(update_supplier_price))
        .route("/approveprice", post(update_summ_price))
        .route("/acceptprice", post(accept_summ_price))
        .route("/createcontract", post(create_contract))
        .route("/createsuppcontract", post(create_supp_contract))
        .route("/createsupppayments", post(create_supp_payments))
        .route("/createsuppdelivery", post(create_supp_delivery))
        .route("/createpostprocesses", post(create_post_processes))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    //let listener = tokio::net::TcpListener::bind("localhost:8080").await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
