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
pub struct VirtualCircuitUpdateInput {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// A Virtual Network record UUID or the VNID of a Virtual Network in your project.
    #[serde(rename = "vnid", skip_serializing_if = "Option::is_none")]
    pub vnid: Option<String>,
}

impl VirtualCircuitUpdateInput {
    pub fn new() -> VirtualCircuitUpdateInput {
        VirtualCircuitUpdateInput {
            description: None,
            name: None,
            speed: None,
            tags: None,
            vnid: None,
        }
    }
}


