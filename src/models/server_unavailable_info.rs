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
pub struct ServerUnavailableInfo {
    #[serde(rename = "facility", skip_serializing_if = "Option::is_none")]
    pub facility: Option<String>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

impl ServerUnavailableInfo {
    pub fn new() -> ServerUnavailableInfo {
        ServerUnavailableInfo {
            facility: None,
            plan: None,
            quantity: None,
            available: None,
        }
    }
}


