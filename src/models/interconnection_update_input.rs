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
pub struct InterconnectionUpdateInput {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "contact_email", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl InterconnectionUpdateInput {
    pub fn new() -> InterconnectionUpdateInput {
        InterconnectionUpdateInput {
            name: None,
            description: None,
            contact_email: None,
            tags: None,
        }
    }
}

