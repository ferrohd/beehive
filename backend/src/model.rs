use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use bb8_postgres::{tokio_postgres, tokio_postgres::Row};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub reputation: i32,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<&Row> for User {
    type Error = tokio_postgres::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.try_get("uuid")?,
            reputation: row.try_get("reputation")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coupon {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub code: String,
    pub description: String,
    pub expiry: Option<DateTime<Utc>>,
    pub domain: String,
    pub score: i32,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<&Row> for Coupon {
    type Error = tokio_postgres::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Coupon {
            id: row.try_get("id")?,
            creator_id: row.try_get("creator_id")?,
            code: row.try_get("code")?,
            description: row.try_get("description")?,
            expiry: row.try_get("expiry")?,
            domain: row.try_get("domain")?,
            score: row.try_get("score")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Serialize)]
pub struct CouponPublic {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub expiry: Option<DateTime<Utc>>,
    pub domain: String,
    pub score: i32,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<&Row> for CouponPublic {
    type Error = tokio_postgres::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(CouponPublic {
            id: row.try_get("id")?,
            code: row.try_get("code")?,
            description: row.try_get("description")?,
            expiry: row.try_get("expiry")?,
            domain: row.try_get("domain")?,
            score: row.try_get("score")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

impl From<Coupon> for CouponPublic {
    fn from(coupon: Coupon) -> Self {
        CouponPublic {
            id: coupon.id,
            code: coupon.code,
            description: coupon.description,
            expiry: coupon.expiry,
            domain: coupon.domain,
            score: coupon.score,
            created_at: coupon.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
    pub id: Uuid,
    pub voter_id: Uuid,
    pub coupon_id: Uuid,
    pub vote_type: bool,
}

impl TryFrom<&Row> for Vote {
    type Error = tokio_postgres::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Vote {
            id: row.try_get("id")?,
            voter_id: row.try_get("voter_id")?,
            coupon_id: row.try_get("coupon_id")?,
            vote_type: row.try_get("vote_type")?,
        })
    }
}
