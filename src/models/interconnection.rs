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
pub struct Interconnection {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "contact_email", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "redundancy", skip_serializing_if = "Option::is_none")]
    pub redundancy: Option<String>,
    /// The connection's speed in bps.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<i32>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<crate::models::InterconnectionPort>>,
    #[serde(rename = "facility", skip_serializing_if = "Option::is_none")]
    pub facility: Option<crate::models::Href>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<crate::models::Href>,
}

impl Interconnection {
    pub fn new() -> Interconnection {
        Interconnection {
            id: None,
            name: None,
            description: None,
            contact_email: None,
            status: None,
            _type: None,
            redundancy: None,
            speed: None,
            tags: None,
            ports: None,
            facility: None,
            organization: None,
        }
    }
}


