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
pub struct LicenseCreateInput {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f32>,
    #[serde(rename = "licensee_product_id", skip_serializing_if = "Option::is_none")]
    pub licensee_product_id: Option<String>,
}

impl LicenseCreateInput {
    pub fn new() -> LicenseCreateInput {
        LicenseCreateInput {
            description: None,
            size: None,
            licensee_product_id: None,
        }
    }
}

