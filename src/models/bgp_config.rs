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
pub struct BgpConfig {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "deployment_type", skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "asn", skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "route_object", skip_serializing_if = "Option::is_none")]
    pub route_object: Option<String>,
    #[serde(rename = "md5", skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    #[serde(rename = "max_prefix", skip_serializing_if = "Option::is_none")]
    pub max_prefix: Option<i32>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::Href>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "requested_at", skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<String>,
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    pub session: Option<serde_json::Value>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl BgpConfig {
    pub fn new() -> BgpConfig {
        BgpConfig {
            id: None,
            status: None,
            deployment_type: None,
            asn: None,
            route_object: None,
            md5: None,
            max_prefix: None,
            project: None,
            created_at: None,
            requested_at: None,
            session: None,
            href: None,
        }
    }
}

