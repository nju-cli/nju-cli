mod crypto;
mod http;
mod model;

use anyhow::{Context, Result};
use serde_json::Value;

pub use crypto::{captcha_verification, order_pin};
pub use http::{VenueSession, prepare_session};
pub use model::{
    CampusVenueInfo, OrderInfoRequest, OrderInfoResponse, PayOrderResponse, ReservationItem,
    SubmitOrderRequest, SubmitOrderResponse, VenueInfo, VenueSiteInfo,
};

use crate::{
    crypto::encrypt_captcha_payload,
    http::{get_api, get_api_typed, post_form_api, post_form_api_typed},
};

pub async fn get_venue_site(session: &VenueSession<'_>, venue_site_id: u64) -> Result<Value> {
    get_api(
        session,
        &format!("/api/front/website/venue_sites/{venue_site_id}"),
        &[],
    )
    .await
}

pub async fn get_reservation_day(
    session: &VenueSession<'_>,
    venue_site_id: u64,
    search_date: &str,
    has_reserve_info: bool,
) -> Result<Value> {
    let mut query = vec![
        ("venueSiteId", venue_site_id.to_string()),
        ("searchDate", search_date.to_string()),
    ];
    if has_reserve_info {
        query.push(("hasReserveInfo", "1".to_string()));
    }

    get_api(session, "/api/reservation/day/info", &query).await
}

pub async fn list_campus_venue_info(
    session: &VenueSession<'_>,
    sport_type: Option<u64>,
) -> Result<CampusVenueInfo> {
    let mut query = Vec::new();
    push_opt(&mut query, "sportType", sport_type);

    get_api_typed(session, "/api/reservation/campus/venue/info", &query).await
}

pub async fn get_order_info(
    session: &VenueSession<'_>,
    request: &OrderInfoRequest,
) -> Result<OrderInfoResponse> {
    post_form_api_typed(session, "/api/reservation/order/info", request).await
}

pub async fn submit_order(
    session: &VenueSession<'_>,
    request: &SubmitOrderRequest,
) -> Result<SubmitOrderResponse> {
    post_form_api_typed(session, "/api/reservation/order/submit", request).await
}

pub async fn pay_order(session: &VenueSession<'_>, trade_no: &str) -> Result<PayOrderResponse> {
    let form = [("venueTradeNo", trade_no), ("isApp", "0")];
    post_form_api_typed(session, "/api/venue/finances/order/pay", &form).await
}

pub async fn list_orders(session: &VenueSession<'_>, page: u64, size: u64) -> Result<Value> {
    get_api(
        session,
        "/api/orders/mine",
        &[("page", page.to_string()), ("size", size.to_string())],
    )
    .await
}

pub async fn get_order(session: &VenueSession<'_>, order_id: u64) -> Result<Value> {
    get_api(session, &format!("/api/orders/{order_id}"), &[]).await
}

pub async fn cancel_order(
    session: &VenueSession<'_>,
    trade_no: &str,
    remark: &str,
) -> Result<Value> {
    let form = [("venueTradeNo", trade_no), ("remark", remark)];
    post_form_api(session, "/api/venue/finances/order/cancel", &form).await
}

pub async fn get_captcha(session: &VenueSession<'_>, client_uid: &str) -> Result<Value> {
    get_api(
        session,
        "/api/captcha/get",
        &[
            ("captchaType", "clickWord".to_string()),
            ("clientUid", client_uid.to_string()),
        ],
    )
    .await
}

pub async fn check_captcha(
    session: &VenueSession<'_>,
    token: &str,
    secret_key: Option<&str>,
    points_json: &str,
) -> Result<Value> {
    let point_json = match secret_key {
        Some(secret_key) => encrypt_captcha_payload(points_json, secret_key)?,
        None => points_json.to_string(),
    };
    let form = [
        ("captchaType", "clickWord"),
        ("pointJson", point_json.as_str()),
        ("token", token),
    ];

    post_form_api(session, "/api/captcha/check", &form).await
}

pub fn reservation_order_json(items: &[ReservationItem]) -> Result<String> {
    serde_json::to_string(items).context("failed to serialize reservation order json")
}

fn push_opt(query: &mut Vec<(&str, String)>, key: &'static str, value: Option<u64>) {
    if let Some(value) = value {
        query.push((key, value.to_string()));
    }
}
