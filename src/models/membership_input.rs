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
pub struct MembershipInput {
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Vec<String>>,
}

impl MembershipInput {
    pub fn new() -> MembershipInput {
        MembershipInput {
            role: None,
        }
    }
}


