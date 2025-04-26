use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::AppState;
use crate::query::ReqItem;

///////////////////////////
pub async fn targets(State(app_state): State<AppState>, ) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let raw: Vec<String> =app_state.db.q_targets().await;
    Ok(Json(raw))
}

pub async fn clients(State(app_state): State<AppState>, ) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let raw: Vec<String> =app_state.db.q_clients().await;
    Ok(Json(raw))
}
pub async fn imo(Path(imo): Path<String>, State(app_state): State<AppState>, ) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let raw: Vec<String> =app_state.db.q_imo_by_target(&imo).await;
    Ok(Json(raw))
}

pub async fn target_place(Path((a, b)): Path<(String, String)>, State(app_state): State<AppState>, ) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let raw: Vec<String> =app_state.db.q_place_by_client_target(&a,&b).await;
    Ok(Json(raw))
}

pub async fn catalogs(Path(target): Path<String>, State(app_state): State<AppState>, ) -> Result<Json< Vec<(String, String)> >, (StatusCode, String)> {
    let raw:  Vec<(String, String)>  =app_state.db.catalogs(&target).await;
    Ok(Json(raw))
}

pub async fn parts_by_cat(Path((a, b)): Path<(String, String)>, State(app_state): State<AppState>, ) -> Result<Json< Vec<(String, String)>>, (StatusCode, String)> {
    let raw: Vec<(String, String)> =app_state.db.parts_by_catalog(&a,&b).await;
    Ok(Json(raw))
}

pub async fn serials_by_cat(Path((a, b,c)): Path<(String,String,String)>, State(app_state): State<AppState>, ) -> Result<Json< Vec<String>>, (StatusCode, String)> {
    let raw: Vec<String> =app_state.db.serials_by_target_catalog(&a,&b,&c).await;
    Ok(Json(raw))
}

////////////////////////////////
pub async fn add_draft(State(app_state): State<AppState>,draft:Json<Vec<ReqItem>>,  )->impl IntoResponse {
    let mut drafts: Vec<ReqItem> =draft.to_vec();
    let mut part_no=1;
    drafts.iter_mut().for_each(|d|{
        d.status=10;
        d.part_no=part_no.to_string();
        part_no+=1;
    });
    app_state.db.q_add_drafts(drafts).await;
    (StatusCode::OK, "Draft added")
}
////////////////////////////////

/////////PROCESSSING//////////
pub async fn requests_by_statuses(Path(a): Path<i32>, State(app_state): State<AppState>, ) -> Result<Json< Vec<(String, String, String, i32)>>, (StatusCode, String)>{
    let raw: Vec<(String, String, String, i32)> =app_state.db.q_requests_by_status(&a).await;
    Ok(Json(raw))
}

pub async fn requests_by_orderid(Path(a): Path<String>, State(app_state): State<AppState>, ) -> Result<Json< Vec<ReqItem>>, (StatusCode, String)>{
    let raw: Vec<ReqItem> =app_state.db.q_requests_by_orderid(a).await;
    Ok(Json(raw))
}


////////PROCESSING////////////