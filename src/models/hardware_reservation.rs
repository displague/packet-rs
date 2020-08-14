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
pub struct HardwareReservation {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "short_id", skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
    #[serde(rename = "facility", skip_serializing_if = "Option::is_none")]
    pub facility: Option<crate::models::Facility>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<crate::models::Plan>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::Project>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<crate::models::Device>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "remove_at", skip_serializing_if = "Option::is_none")]
    pub remove_at: Option<String>,
}

impl HardwareReservation {
    pub fn new() -> HardwareReservation {
        HardwareReservation {
            id: None,
            short_id: None,
            facility: None,
            plan: None,
            href: None,
            project: None,
            device: None,
            created_at: None,
            remove_at: None,
        }
    }
}


