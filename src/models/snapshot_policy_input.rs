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
pub struct SnapshotPolicyInput {
    #[serde(rename = "snapshot_count", skip_serializing_if = "Option::is_none")]
    pub snapshot_count: Option<i32>,
    #[serde(rename = "snapshot_frequency", skip_serializing_if = "Option::is_none")]
    pub snapshot_frequency: Option<String>,
}

impl SnapshotPolicyInput {
    pub fn new() -> SnapshotPolicyInput {
        SnapshotPolicyInput {
            snapshot_count: None,
            snapshot_frequency: None,
        }
    }
}


