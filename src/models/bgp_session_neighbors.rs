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
pub struct BgpSessionNeighbors {
    /// A list of BGP session neighbor data
    #[serde(rename = "bgp_neighbors", skip_serializing_if = "Option::is_none")]
    pub bgp_neighbors: Option<Vec<crate::models::BgpNeighborData>>,
}

impl BgpSessionNeighbors {
    pub fn new() -> BgpSessionNeighbors {
        BgpSessionNeighbors {
            bgp_neighbors: None,
        }
    }
}


