use axum::{
    extract::{Path, State, ws::{WebSocket, WebSocketUpgrade}},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

use crate::types::{StrategyType, SignalType};

// ============================================================================
// API State
// ============================================================================

#[derive(Clone)]
pub struct ApiState {
    pub delegations: Arc<RwLock<Vec<DelegationInfo>>>,
    pub positions: Arc<RwLock<Vec<PositionInfo>>>,
    pub stats: Arc<RwLock<BotStats>>,
}

impl ApiState {
    pub fn new() -> Self {
        Self {
            delegations: Arc::new(RwLock::new(Vec::new())),
            positions: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(BotStats::default())),
        }
    }
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationInfo {
    pub user: String,
    pub strategy: StrategyType,
    pub max_position_size_sol: f64,
    pub max_concurrent_trades: u8,
    pub is_active: bool,
    pub active_trades: u8,
    pub total_trades: u64,
    pub profitable_trades: u64,
    pub total_pnl: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionInfo {
    pub position_id: String,
    pub user: String,
    pub token_mint: String,
    pub token_symbol: String,
    pub amount_sol: f64,
    pub entry_price: u64,
    pub current_price: u64,
    pub take_profit_price: u64,
    pub stop_loss_price: u64,
    pub status: String,
    pub pnl: i64,
    pub pnl_percentage: f64,
    pub opened_at: i64,
    pub closed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BotStats {
    pub is_running: bool,
    pub uptime_seconds: u64,
    pub total_scans: u64,
    pub tokens_analyzed: u64,
    pub signals_generated: u64,
    pub trades_executed: u64,
    pub last_scan_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub risk_level: String,
    pub target_return: String,
    pub win_rate: String,
    pub hold_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub wallet: String,
    pub strategy: String,
    pub is_active: bool,
    pub active_positions: u8,
    pub total_trades: u64,
    pub profitable_trades: u64,
    pub win_rate: f64,
    pub total_pnl_sol: f64,
    pub total_pnl_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

// ============================================================================
// API Server
// ============================================================================

pub async fn start_api_server(state: ApiState, port: u16) -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/strategies", get(strategies_handler))
        .route("/api/users/:wallet/positions", get(user_positions_handler))
        .route("/api/users/:wallet/stats", get(user_stats_handler))
        .route("/api/positions", get(all_positions_handler))
        .route("/api/stats", get(bot_stats_handler))
        .route("/api/stream", get(websocket_handler))
        .layer(cors)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    info!("🚀 API server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ============================================================================
// Route Handlers
// ============================================================================

async fn health_handler(
    State(state): State<ApiState>,
) -> Json<HealthResponse> {
    let stats = state.stats.read().await;

    Json(HealthResponse {
        status: if stats.is_running { "healthy" } else { "stopped" },
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: stats.uptime_seconds,
    })
}

async fn strategies_handler() -> Json<Vec<StrategyInfo>> {
    Json(vec![
        StrategyInfo {
            id: "conservative".to_string(),
            name: "Conservative Multi-Factor".to_string(),
            description: "Balanced approach with 30-70% bonding curve. 2x target, 60-70% win rate.".to_string(),
            risk_level: "Medium".to_string(),
            target_return: "2x".to_string(),
            win_rate: "60-70%".to_string(),
            hold_time: "1 hour".to_string(),
        },
        StrategyInfo {
            id: "ultra_early_sniper".to_string(),
            name: "Ultra-Early Sniper".to_string(),
            description: "Catch tokens in first 5 minutes. <10% bonding curve. 3-10x moonshot potential.".to_string(),
            risk_level: "Very High".to_string(),
            target_return: "3-10x".to_string(),
            win_rate: "30-40%".to_string(),
            hold_time: "10 minutes".to_string(),
        },
        StrategyInfo {
            id: "momentum_scalper".to_string(),
            name: "Momentum Scalper".to_string(),
            description: "Ride explosive momentum waves. 40-80% bonding curve. Quick 1.5x flips.".to_string(),
            risk_level: "High".to_string(),
            target_return: "1.5x".to_string(),
            win_rate: "50-60%".to_string(),
            hold_time: "30 minutes".to_string(),
        },
        StrategyInfo {
            id: "graduation_anticipator".to_string(),
            name: "Graduation Anticipator".to_string(),
            description: "Position before DEX migration. 60-85% bonding curve. Steady 1.8x gains.".to_string(),
            risk_level: "Low".to_string(),
            target_return: "1.8x".to_string(),
            win_rate: "70-80%".to_string(),
            hold_time: "2 hours".to_string(),
        },
    ])
}

async fn user_positions_handler(
    State(state): State<ApiState>,
    Path(wallet): Path<String>,
) -> Result<Json<Vec<PositionInfo>>, (StatusCode, Json<ErrorResponse>)> {
    let positions = state.positions.read().await;

    let user_positions: Vec<PositionInfo> = positions
        .iter()
        .filter(|p| p.user == wallet)
        .cloned()
        .collect();

    Ok(Json(user_positions))
}

async fn user_stats_handler(
    State(state): State<ApiState>,
    Path(wallet): Path<String>,
) -> Result<Json<UserStats>, (StatusCode, Json<ErrorResponse>)> {
    let delegations = state.delegations.read().await;

    let delegation = delegations
        .iter()
        .find(|d| d.user == wallet)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Delegation not found".to_string(),
                }),
            )
        })?;

    let win_rate = if delegation.total_trades > 0 {
        (delegation.profitable_trades as f64 / delegation.total_trades as f64) * 100.0
    } else {
        0.0
    };

    let pnl_sol = delegation.total_pnl as f64 / 1_000_000_000.0; // lamports to SOL
    let pnl_usd = pnl_sol * 100.0; // Approximate SOL price

    Ok(Json(UserStats {
        wallet: delegation.user.clone(),
        strategy: format!("{:?}", delegation.strategy),
        is_active: delegation.is_active,
        active_positions: delegation.active_trades,
        total_trades: delegation.total_trades,
        profitable_trades: delegation.profitable_trades,
        win_rate,
        total_pnl_sol: pnl_sol,
        total_pnl_usd: pnl_usd,
    }))
}

async fn all_positions_handler(
    State(state): State<ApiState>,
) -> Json<Vec<PositionInfo>> {
    let positions = state.positions.read().await;
    Json(positions.clone())
}

async fn bot_stats_handler(
    State(state): State<ApiState>,
) -> Json<BotStats> {
    let stats = state.stats.read().await;
    Json(stats.clone())
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<ApiState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(mut socket: WebSocket, state: ApiState) {
    info!("WebSocket connection established");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Send stats update
        let stats = state.stats.read().await;
        let message = serde_json::to_string(&*stats).unwrap();

        if socket.send(axum::extract::ws::Message::Text(message)).await.is_err() {
            warn!("WebSocket connection closed");
            break;
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

impl ApiState {
    pub async fn update_bot_stats(&self, is_running: bool, uptime: u64, scans: u64, analyzed: u64, signals: u64, trades: u64) {
        let mut stats = self.stats.write().await;
        stats.is_running = is_running;
        stats.uptime_seconds = uptime;
        stats.total_scans = scans;
        stats.tokens_analyzed = analyzed;
        stats.signals_generated = signals;
        stats.trades_executed = trades;
        stats.last_scan_at = Some(chrono::Utc::now().timestamp());
    }

    pub async fn add_delegation(&self, delegation: DelegationInfo) {
        let mut delegations = self.delegations.write().await;
        delegations.push(delegation);
    }

    pub async fn update_delegation(&self, user: &str, is_active: bool, active_trades: u8, total_trades: u64, profitable_trades: u64, total_pnl: i64) {
        let mut delegations = self.delegations.write().await;
        if let Some(delegation) = delegations.iter_mut().find(|d| d.user == user) {
            delegation.is_active = is_active;
            delegation.active_trades = active_trades;
            delegation.total_trades = total_trades;
            delegation.profitable_trades = profitable_trades;
            delegation.total_pnl = total_pnl;
        }
    }

    pub async fn add_position(&self, position: PositionInfo) {
        let mut positions = self.positions.write().await;
        positions.push(position);
    }

    pub async fn update_position(&self, position_id: &str, current_price: u64, status: &str, pnl: i64, closed_at: Option<i64>) {
        let mut positions = self.positions.write().await;
        if let Some(position) = positions.iter_mut().find(|p| p.position_id == position_id) {
            position.current_price = current_price;
            position.status = status.to_string();
            position.pnl = pnl;
            position.closed_at = closed_at;

            // Calculate PnL percentage
            if position.entry_price > 0 {
                position.pnl_percentage = ((current_price as f64 - position.entry_price as f64) / position.entry_price as f64) * 100.0;
            }
        }
    }
}
