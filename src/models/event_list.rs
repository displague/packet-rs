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
pub struct EventList {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::Event>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::models::Meta>,
}

impl EventList {
    pub fn new() -> EventList {
        EventList {
            events: None,
            meta: None,
        }
    }
}


