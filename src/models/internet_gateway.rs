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
pub struct InternetGateway {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "virtual_network", skip_serializing_if = "Option::is_none")]
    pub virtual_network: Option<crate::models::Href>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "ip_reservations", skip_serializing_if = "Option::is_none")]
    pub ip_reservations: Option<Vec<crate::models::Href>>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl InternetGateway {
    pub fn new() -> InternetGateway {
        InternetGateway {
            id: None,
            created_at: None,
            virtual_network: None,
            created_by: None,
            ip_reservations: None,
            href: None,
        }
    }
}


