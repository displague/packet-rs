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
pub struct DeviceUpdateInput {
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "billing_cycle", skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(rename = "userdata", skip_serializing_if = "Option::is_none")]
    pub userdata: Option<String>,
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "always_pxe", skip_serializing_if = "Option::is_none")]
    pub always_pxe: Option<bool>,
    #[serde(rename = "ipxe_script_url", skip_serializing_if = "Option::is_none")]
    pub ipxe_script_url: Option<String>,
    #[serde(rename = "spot_instance", skip_serializing_if = "Option::is_none")]
    pub spot_instance: Option<bool>,
    #[serde(rename = "customdata", skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,
}

impl DeviceUpdateInput {
    pub fn new() -> DeviceUpdateInput {
        DeviceUpdateInput {
            hostname: None,
            description: None,
            billing_cycle: None,
            userdata: None,
            locked: None,
            tags: None,
            always_pxe: None,
            ipxe_script_url: None,
            spot_instance: None,
            customdata: None,
        }
    }
}

