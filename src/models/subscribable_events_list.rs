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
pub struct SubscribableEventsList {
    #[serde(rename = "subscribable_events", skip_serializing_if = "Option::is_none")]
    pub subscribable_events: Option<Vec<crate::models::SubscribableEvent>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::models::Meta>,
}

impl SubscribableEventsList {
    pub fn new() -> SubscribableEventsList {
        SubscribableEventsList {
            subscribable_events: None,
            meta: None,
        }
    }
}


