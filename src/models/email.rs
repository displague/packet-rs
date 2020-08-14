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
pub struct Email {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl Email {
    pub fn new() -> Email {
        Email {
            id: None,
            address: None,
            default: None,
            verified: None,
            href: None,
        }
    }
}

