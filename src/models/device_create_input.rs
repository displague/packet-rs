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
pub struct DeviceCreateInput {
    #[serde(rename = "facility")]
    pub facility: String,
    #[serde(rename = "plan")]
    pub plan: String,
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "billing_cycle", skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(rename = "operating_system")]
    pub operating_system: String,
    #[serde(rename = "always_pxe", skip_serializing_if = "Option::is_none")]
    pub always_pxe: Option<bool>,
    #[serde(rename = "ipxe_script_url", skip_serializing_if = "Option::is_none")]
    pub ipxe_script_url: Option<String>,
    #[serde(rename = "userdata", skip_serializing_if = "Option::is_none")]
    pub userdata: Option<String>,
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "customdata", skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,
    #[serde(rename = "hardware_reservation_id", skip_serializing_if = "Option::is_none")]
    pub hardware_reservation_id: Option<String>,
    #[serde(rename = "spot_instance", skip_serializing_if = "Option::is_none")]
    pub spot_instance: Option<bool>,
    #[serde(rename = "spot_price_max", skip_serializing_if = "Option::is_none")]
    pub spot_price_max: Option<f32>,
    #[serde(rename = "termination_time", skip_serializing_if = "Option::is_none")]
    pub termination_time: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "project_ssh_keys", skip_serializing_if = "Option::is_none")]
    pub project_ssh_keys: Option<Vec<String>>,
    #[serde(rename = "user_ssh_keys", skip_serializing_if = "Option::is_none")]
    pub user_ssh_keys: Option<Vec<String>>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "public_ipv4_subnet_size", skip_serializing_if = "Option::is_none")]
    pub public_ipv4_subnet_size: Option<f32>,
    #[serde(rename = "private_ipv4_subnet_size", skip_serializing_if = "Option::is_none")]
    pub private_ipv4_subnet_size: Option<f32>,
    #[serde(rename = "ip_addresses", skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<crate::models::InstancesBatchCreateInputIpAddresses>>,
}

impl DeviceCreateInput {
    pub fn new(facility: String, plan: String, operating_system: String) -> DeviceCreateInput {
        DeviceCreateInput {
            facility,
            plan,
            hostname: None,
            description: None,
            billing_cycle: None,
            operating_system,
            always_pxe: None,
            ipxe_script_url: None,
            userdata: None,
            locked: None,
            customdata: None,
            hardware_reservation_id: None,
            spot_instance: None,
            spot_price_max: None,
            termination_time: None,
            tags: None,
            project_ssh_keys: None,
            user_ssh_keys: None,
            features: None,
            public_ipv4_subnet_size: None,
            private_ipv4_subnet_size: None,
            ip_addresses: None,
        }
    }
}


