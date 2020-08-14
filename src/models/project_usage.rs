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
pub struct ProjectUsage {
    #[serde(rename = "facility", skip_serializing_if = "Option::is_none")]
    pub facility: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "plan_version", skip_serializing_if = "Option::is_none")]
    pub plan_version: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
}

impl ProjectUsage {
    pub fn new() -> ProjectUsage {
        ProjectUsage {
            facility: None,
            _type: None,
            name: None,
            plan: None,
            plan_version: None,
            quantity: None,
            unit: None,
            price: None,
            total: None,
        }
    }
}


