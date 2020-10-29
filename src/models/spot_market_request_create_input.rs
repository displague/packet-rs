/*
 * Metal API
 *
 * This is the API for Equinix Metal Product. Interact with your devices, user account, and projects.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@equinixmetal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpotMarketRequestCreateInput {
    #[serde(rename = "instance_attributes", skip_serializing_if = "Option::is_none")]
    pub instance_attributes: Option<crate::models::SpotMarketRequestCreateInputInstanceAttributes>,
    #[serde(rename = "devices_min", skip_serializing_if = "Option::is_none")]
    pub devices_min: Option<i32>,
    #[serde(rename = "devices_max", skip_serializing_if = "Option::is_none")]
    pub devices_max: Option<i32>,
    #[serde(rename = "max_bid_price", skip_serializing_if = "Option::is_none")]
    pub max_bid_price: Option<f32>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
    #[serde(rename = "facilities", skip_serializing_if = "Option::is_none")]
    pub facilities: Option<Vec<String>>,
}

impl SpotMarketRequestCreateInput {
    pub fn new() -> SpotMarketRequestCreateInput {
        SpotMarketRequestCreateInput {
            instance_attributes: None,
            devices_min: None,
            devices_max: None,
            max_bid_price: None,
            end_at: None,
            facilities: None,
        }
    }
}


