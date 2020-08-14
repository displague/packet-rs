/*
 * Packet API
 *
 * This is the API for Packet. Interact with your devices, user account, and projects.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@packet.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodUpdateInput {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "cardholder_name", skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[serde(rename = "expiration_month", skip_serializing_if = "Option::is_none")]
    pub expiration_month: Option<String>,
    #[serde(rename = "expiration_year", skip_serializing_if = "Option::is_none")]
    pub expiration_year: Option<i32>,
    #[serde(rename = "billing_address", skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<serde_json::Value>,
}

impl PaymentMethodUpdateInput {
    pub fn new() -> PaymentMethodUpdateInput {
        PaymentMethodUpdateInput {
            name: None,
            default: None,
            cardholder_name: None,
            expiration_month: None,
            expiration_year: None,
            billing_address: None,
        }
    }
}

