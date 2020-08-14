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
pub struct VolumeAttachment {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<crate::models::Href>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<crate::models::Href>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl VolumeAttachment {
    pub fn new() -> VolumeAttachment {
        VolumeAttachment {
            id: None,
            created_at: None,
            volume: None,
            device: None,
            href: None,
        }
    }
}

