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
pub struct HardwareLocation {
    #[serde(rename = "cage", skip_serializing_if = "Option::is_none")]
    pub cage: Option<String>,
    #[serde(rename = "facility", skip_serializing_if = "Option::is_none")]
    pub facility: Option<String>,
    #[serde(rename = "rack", skip_serializing_if = "Option::is_none")]
    pub rack: Option<String>,
    #[serde(rename = "row", skip_serializing_if = "Option::is_none")]
    pub row: Option<String>,
    #[serde(rename = "switch", skip_serializing_if = "Option::is_none")]
    pub switch: Option<String>,
}

impl HardwareLocation {
    pub fn new() -> HardwareLocation {
        HardwareLocation {
            cage: None,
            facility: None,
            rack: None,
            row: None,
            switch: None,
        }
    }
}

