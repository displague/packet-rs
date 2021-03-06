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
pub struct VirtualCircuitList {
    #[serde(rename = "virtual_circuits", skip_serializing_if = "Option::is_none")]
    pub virtual_circuits: Option<Vec<crate::models::VirtualCircuit>>,
}

impl VirtualCircuitList {
    pub fn new() -> VirtualCircuitList {
        VirtualCircuitList {
            virtual_circuits: None,
        }
    }
}


