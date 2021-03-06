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
pub struct SubscribableEvent {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "event_type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "event_name", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "event_slug", skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
}

impl SubscribableEvent {
    pub fn new() -> SubscribableEvent {
        SubscribableEvent {
            id: None,
            event_type: None,
            event_name: None,
            event_slug: None,
        }
    }
}


