use axum::extract::FromRequestParts;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::model::CouponPublic;
use crate::model::Vote;
use crate::model::{Coupon, User};
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user", post(create_user_handler))
        .route("/user", get(get_user_handler))
        .route("/coupon", post(create_coupon_handler))
        .route("/coupon", get(get_coupons_handler))
        .route("/coupon/{coupon_id}/vote", post(vote_coupon_handler))
}

async fn create_user_handler(
    State(state): State<AppState>,
) -> Result<Json<User>, (StatusCode, String)> {
    let client = state.db_client.get().await.map_err(internal_error)?;
    let user = User {
        id: Uuid::now_v7(),
        reputation: 5,
        created_at: Utc::now(),
    };
    let row = client
        .query_one(
            "INSERT INTO users (uuid, reputation, created_at)
             VALUES ($1, $2, $3)
             RETURNING *",
            &[&user.id, &user.reputation],
        )
        .await
        .map_err(internal_error)?;
    let user = User::try_from(&row).map_err(internal_error)?;

    Ok(Json(user))
}

async fn get_user_handler(
    State(state): State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<User>, (StatusCode, String)> {
    let client = state.db_client.get().await.map_err(internal_error)?;

    let row = client
        .query_one("SELECT * FROM users WHERE uuid = $1", &[&user_id])
        .await
        .map_err(internal_error)?;
    let user = User::try_from(&row).map_err(internal_error)?;

    Ok(Json(user))
}

#[derive(Deserialize)]
struct CouponCreateBody {
    code: String,
    description: String,
    expiry: Option<String>,
    domain: String,
}
impl From<(CouponCreateBody, Uuid)> for Coupon {
    fn from((body, creator_id): (CouponCreateBody, Uuid)) -> Self {
        Coupon {
            id: Uuid::now_v7(),
            creator_id,
            code: body.code,
            description: body.description,
            expiry: body.expiry.map(|s| s.parse().unwrap()),
            domain: body.domain,
            score: 0,
            created_at: Utc::now(),
        }
    }
}
async fn create_coupon_handler(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Json(payload): Json<CouponCreateBody>,
) -> Result<Json<CouponPublic>, (StatusCode, String)> {
    let mut client = state.db_client.get().await.map_err(internal_error)?;

    let row = client
        .query_one("SELECT * FROM users WHERE uuid = $1", &[&user_id])
        .await
        .map_err(internal_error)?;
    let mut user = User::try_from(&row).map_err(internal_error)?;

    if user.reputation < state.config.min_coupon_create_reputation {
        return Err((
            StatusCode::FORBIDDEN,
            "User does not have enough reputation".to_string(),
        ));
    }

    let transaction = client.transaction().await.map_err(internal_error)?;

    user.reputation -= state.config.create_coupon_cost;
    transaction
        .execute(
            "UPDATE users SET reputation = $1 WHERE uuid = $2",
            &[&user.reputation, &user.id],
        )
        .await
        .map_err(internal_error)?;

    let coupon = Coupon::from((payload, user.id));
    let row = transaction
        .query_one(
            "INSERT INTO coupons (id, creator_id, code, description, expiry, domain, score, created_at)
             VALUES ($1, $2, $3 $4, $5, $6, $7, $8)
             RETURNING *",
            &[
                &coupon.id,
                &coupon.creator_id,
                &coupon.code,
                &coupon.description,
                &coupon.expiry,
                &coupon.domain,
                &coupon.score,
                &coupon.created_at,
            ],
        )
        .await
        .map_err(internal_error)?;

    transaction.commit().await.map_err(internal_error)?;
    let coupon = CouponPublic::try_from(&row).map_err(internal_error)?;

    Ok(Json(coupon))
}

#[derive(Debug, Deserialize)]
struct CouponDomainQuery {
    domain: String,
}
async fn get_coupons_handler(
    State(state): State<AppState>,
    Query(params): Query<CouponDomainQuery>,
) -> Result<Json<Vec<CouponPublic>>, (StatusCode, String)> {
    let client = state.db_client.get().await.map_err(internal_error)?;

    let rows = client
        .query(
            "SELECT *
             FROM coupons
             WHERE domain = $1 AND expiry > NOW()
             ORDER BY score DESC
             LIMIT 20",
            &[&params.domain],
        )
        .await
        .map_err(internal_error)?;

    let coupons = rows
        .iter()
        .map(|row| CouponPublic::try_from(row).map_err(internal_error))
        .collect::<Result<Vec<CouponPublic>, (StatusCode, String)>>()?;

    Ok(Json(coupons))
}

#[derive(Deserialize)]
struct VoteRequest {
    vote_type: bool,
}
async fn vote_coupon_handler(
    State(state): State<AppState>,
    Path(coupon_id): Path<Uuid>,
    UserId(user_id): UserId,
    Json(payload): Json<VoteRequest>,
) -> Result<Json<CouponPublic>, (StatusCode, String)> {
    let mut client = state.db_client.get().await.map_err(internal_error)?;

    let row = client
        .query_one("SELECT * FROM users WHERE uuid = $1", &[&user_id])
        .await
        .map_err(internal_error)?;
    let mut user = User::try_from(&row).map_err(internal_error)?;

    if user.reputation < state.config.vote_coupon_cost {
        return Err((
            StatusCode::FORBIDDEN,
            "User does not have enough reputation to vote".to_string(),
        ));
    }

    let row = client
        .query_one("SELECT * FROM coupons WHERE id = $1", &[&coupon_id])
        .await
        .map_err(internal_error)?;
    let _coupon = Coupon::try_from(&row).map_err(internal_error)?;

    let transaction = client.transaction().await.map_err(internal_error)?;

    // Update user reputation
    user.reputation -= state.config.vote_coupon_cost;
    transaction
        .execute(
            "UPDATE users SET reputation = $1 WHERE uuid = $2",
            &[&user.reputation, &user.id],
        )
        .await
        .map_err(internal_error)?;

    // Insert vote
    let row = transaction
        .query_one(
            "INSERT INTO votes (coupon_id, voter_uuid, vote_type)
             VALUES ($1, $2, $3)
             RETURNING *",
            &[&Uuid::now_v7(), &coupon_id, &user_id, &payload.vote_type],
        )
        .await
        .map_err(internal_error)?;
    let vote = Vote::try_from(&row).map_err(internal_error)?;

    // Update coupon score
    let vote_value: i32 = if payload.vote_type { 1 } else { -1 };
    let row = transaction
        .query_one(
            "UPDATE coupons
             SET score = score + $1
             WHERE id = $2
             RETURNING *",
            &[&vote_value, &vote.coupon_id],
        )
        .await
        .map_err(internal_error)?;

    transaction.commit().await.map_err(internal_error)?;

    let coupon = CouponPublic::try_from(&row).map_err(internal_error)?;

    Ok(Json(coupon))
}

fn internal_error<E: std::error::Error>(err: E) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal error: {}", err),
    )
}

struct UserId(Uuid);
impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = parts.headers.get("x-user-id").ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing x-user-id header".to_string(),
        ))?;

        let user_id = header
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid x-user-id header value".to_string(),
                )
            })?
            .parse()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid x-user-id header value".to_string(),
                )
            })?;

        Ok(UserId(user_id))
    }
}
