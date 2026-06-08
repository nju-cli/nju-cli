use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ApiResponse<T> {
    pub code: i64,
    pub data: Option<T>,
    pub message: Option<String>,
    #[serde(rename = "messageEn")]
    pub message_en: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReservationItem {
    #[serde(rename = "spaceId")]
    pub space_id: String,
    #[serde(rename = "timeId")]
    pub time_id: String,
    #[serde(rename = "venueSpaceGroupId", skip_serializing_if = "Option::is_none")]
    pub venue_space_group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfoRequest {
    pub venue_site_id: u64,
    pub reservation_date: String,
    pub week_start_date: String,
    pub reservation_order_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitOrderRequest {
    pub venue_site_id: u64,
    pub reservation_date: String,
    pub week_start_date: String,
    pub reservation_order_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_pin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_verification: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CampusVenueInfo {
    pub advance_reservation_days: Option<u64>,
    pub reservation_num_max: Option<u64>,
    #[serde(default)]
    pub venue_info: BTreeMap<String, Vec<VenueInfo>>,
    #[serde(default)]
    pub venue_site_info: BTreeMap<String, Vec<VenueSiteInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VenueInfo {
    pub id: Option<u64>,
    pub campus_id: Option<u64>,
    #[serde(default)]
    pub venue_name: String,
    #[serde(default)]
    pub venue_photo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VenueSiteInfo {
    pub id: u64,
    pub campus_id: Option<u64>,
    #[serde(default)]
    pub campus_name: String,
    #[serde(default)]
    pub venue_name: String,
    #[serde(default)]
    pub site_name: String,
    #[serde(default)]
    pub sport_name: Option<String>,
    pub sport_type: Option<u64>,
    #[serde(default)]
    pub open_start_date: Option<String>,
    #[serde(default)]
    pub open_end_date: Option<String>,
    pub space_count: Option<u64>,
    pub reservation_space_count: Option<u64>,
    #[serde(default)]
    pub site_telephone: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfoResponse {
    pub order_price: Option<f64>,
    pub total_amount: Option<f64>,
    pub order_amount: Option<f64>,
    pub amount: Option<f64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl OrderInfoResponse {
    pub fn amount(&self) -> Option<f64> {
        self.order_price
            .or(self.total_amount)
            .or(self.order_amount)
            .or(self.amount)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubmitOrderResponse {
    pub id: Option<u64>,
    pub trade_no: Option<String>,
    pub reservation_start_date: Option<String>,
    pub reservation_end_date: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayOrderResponse {
    pub pay_fee: Option<f64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
