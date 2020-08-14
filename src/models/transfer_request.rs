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
pub struct TransferRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "target_organization", skip_serializing_if = "Option::is_none")]
    pub target_organization: Option<crate::models::Href>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::Href>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl TransferRequest {
    pub fn new() -> TransferRequest {
        TransferRequest {
            id: None,
            created_at: None,
            updated_at: None,
            target_organization: None,
            project: None,
            href: None,
        }
    }
}


