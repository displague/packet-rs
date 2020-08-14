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
pub struct InvitationInput {
    #[serde(rename = "invitee")]
    pub invitee: String,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(rename = "projects_ids", skip_serializing_if = "Option::is_none")]
    pub projects_ids: Option<Vec<String>>,
}

impl InvitationInput {
    pub fn new(invitee: String) -> InvitationInput {
        InvitationInput {
            invitee,
            message: None,
            roles: None,
            projects_ids: None,
        }
    }
}

