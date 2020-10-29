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
pub struct VolumeSnapshotList {
    #[serde(rename = "snapshots", skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<crate::models::VolumeSnapshot>>,
}

impl VolumeSnapshotList {
    pub fn new() -> VolumeSnapshotList {
        VolumeSnapshotList {
            snapshots: None,
        }
    }
}


