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
pub struct Port {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "hardware", skip_serializing_if = "Option::is_none")]
    pub hardware: Option<crate::models::Href>,
    #[serde(rename = "virtual_networks", skip_serializing_if = "Option::is_none")]
    pub virtual_networks: Option<Vec<crate::models::Href>>,
    #[serde(rename = "connected_port", skip_serializing_if = "Option::is_none")]
    pub connected_port: Option<crate::models::Href>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl Port {
    pub fn new() -> Port {
        Port {
            id: None,
            _type: None,
            name: None,
            data: None,
            hardware: None,
            virtual_networks: None,
            connected_port: None,
            href: None,
        }
    }
}


