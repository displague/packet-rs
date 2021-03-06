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
pub struct PortConvertLayer3InputRequestIps {
    #[serde(rename = "address_family", skip_serializing_if = "Option::is_none")]
    pub address_family: Option<i32>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}

impl PortConvertLayer3InputRequestIps {
    pub fn new() -> PortConvertLayer3InputRequestIps {
        PortConvertLayer3InputRequestIps {
            address_family: None,
            public: None,
        }
    }
}


