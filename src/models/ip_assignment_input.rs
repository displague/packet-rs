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
pub struct IpAssignmentInput {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "manageable", skip_serializing_if = "Option::is_none")]
    pub manageable: Option<bool>,
    #[serde(rename = "customdata", skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,
}

impl IpAssignmentInput {
    pub fn new(address: String) -> IpAssignmentInput {
        IpAssignmentInput {
            address,
            manageable: None,
            customdata: None,
        }
    }
}


