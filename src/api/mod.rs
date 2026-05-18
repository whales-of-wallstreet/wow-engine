use axum::{
    routing::{get, post},
    Router, Json, http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::bridge::Chain;
use crate::router::{RoutePlanner, RouteOption};
use crate::anchor::{sep24::Sep24Client, sep38::Sep38Client, Sep24InteractiveResponse, Sep38Quote};

#[derive(Deserialize)]
pub struct QuoteRequest {
    pub source_chain: Chain,
    pub dest_chain: Chain,
    pub source_asset: String,
    pub dest_asset: String,
    pub amount_in: u64,
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub routes: Vec<RouteOption>,
}

#[derive(Deserialize)]
pub struct DepositRequest {
    pub anchor_domain: String,
    pub asset_code: String,
    pub account: String,
}

#[derive(Deserialize)]
pub struct AnchorQuoteRequest {
    pub anchor_domain: String,
    pub sell_asset: String,
    pub buy_asset: String,
    pub sell_amount: f64,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
}

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v1/health", get(health_handler))
        .route("/api/v1/quote", post(quote_handler))
        .route("/api/v1/anchor/deposit", post(deposit_handler))
        .route("/api/v1/anchor/quote", post(anchor_quote_handler))
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "wow-engine",
        version: "0.1.0",
    })
}

async fn quote_handler(
    Json(payload): Json<QuoteRequest>,
) -> Result<Json<QuoteResponse>, (StatusCode, String)> {
    let planner = RoutePlanner::new();
    match planner.find_best_route(
        payload.source_chain,
        payload.dest_chain,
        &payload.source_asset,
        &payload.dest_asset,
        payload.amount_in,
    ).await {
        Ok(routes) => Ok(Json(QuoteResponse { routes })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn deposit_handler(
    Json(payload): Json<DepositRequest>,
) -> Result<Json<Sep24InteractiveResponse>, (StatusCode, String)> {
    let client = Sep24Client::new();
    match client.initiate_deposit(
        &payload.anchor_domain,
        &payload.asset_code,
        &payload.account,
    ).await {
        Ok(tx) => Ok(Json(tx)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn anchor_quote_handler(
    Json(payload): Json<AnchorQuoteRequest>,
) -> Result<Json<Sep38Quote>, (StatusCode, String)> {
    let client = Sep38Client::new();
    match client.get_quote(
        &payload.anchor_domain,
        &payload.sell_asset,
        &payload.buy_asset,
        payload.sell_amount,
    ).await {
        Ok(quote) => Ok(Json(quote)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
