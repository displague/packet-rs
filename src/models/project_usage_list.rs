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
pub struct ProjectUsageList {
    #[serde(rename = "usages", skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<crate::models::ProjectUsage>>,
}

impl ProjectUsageList {
    pub fn new() -> ProjectUsageList {
        ProjectUsageList {
            usages: None,
        }
    }
}


