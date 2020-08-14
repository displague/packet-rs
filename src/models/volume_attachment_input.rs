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
pub struct VolumeAttachmentInput {
    #[serde(rename = "device_id")]
    pub device_id: String,
}

impl VolumeAttachmentInput {
    pub fn new(device_id: String) -> VolumeAttachmentInput {
        VolumeAttachmentInput {
            device_id,
        }
    }
}

