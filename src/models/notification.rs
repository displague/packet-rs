/*
 * Metal API
 *
 * This is the API for Equinix Metal Product. Interact with your devices, user account, and projects.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@equinixmetal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::Href>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl Notification {
    pub fn new() -> Notification {
        Notification {
            id: None,
            _type: None,
            body: None,
            severity: None,
            read: None,
            context: None,
            created_at: None,
            updated_at: None,
            user: None,
            href: None,
        }
    }
}


