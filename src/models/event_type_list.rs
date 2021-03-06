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
pub struct EventTypeList {
    #[serde(rename = "event_types", skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<crate::models::EventType>>,
}

impl EventTypeList {
    pub fn new() -> EventTypeList {
        EventTypeList {
            event_types: None,
        }
    }
}


