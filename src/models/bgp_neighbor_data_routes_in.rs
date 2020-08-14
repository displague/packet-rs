/*
 * Packet API
 *
 * This is the API for Packet. Interact with your devices, user account, and projects.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@packet.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BgpNeighborDataRoutesIn {
    /// A project network
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(rename = "exact", skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

impl BgpNeighborDataRoutesIn {
    pub fn new() -> BgpNeighborDataRoutesIn {
        BgpNeighborDataRoutesIn {
            route: None,
            exact: None,
        }
    }
}

