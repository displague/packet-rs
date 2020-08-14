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
pub struct SpotPricesPerFacility {
    #[serde(rename = "baremetal_2a", skip_serializing_if = "Option::is_none")]
    pub baremetal_2a: Option<crate::models::SpotPricesPerBaremetal>,
    #[serde(rename = "baremetal_2a2", skip_serializing_if = "Option::is_none")]
    pub baremetal_2a2: Option<crate::models::SpotPricesPerBaremetal>,
    #[serde(rename = "baremetal_1", skip_serializing_if = "Option::is_none")]
    pub baremetal_1: Option<crate::models::SpotPricesPerBaremetal>,
    #[serde(rename = "baremetal_3", skip_serializing_if = "Option::is_none")]
    pub baremetal_3: Option<crate::models::SpotPricesPerBaremetal>,
    #[serde(rename = "c2.medium.x86", skip_serializing_if = "Option::is_none")]
    pub c2_medium_x86: Option<crate::models::SpotPricesPerBaremetal>,
    #[serde(rename = "baremetal_2", skip_serializing_if = "Option::is_none")]
    pub baremetal_2: Option<crate::models::SpotPricesPerBaremetal>,
    #[serde(rename = "m2.xlarge.x86", skip_serializing_if = "Option::is_none")]
    pub m2_xlarge_x86: Option<crate::models::SpotPricesPerBaremetal>,
    #[serde(rename = "baremetal_s", skip_serializing_if = "Option::is_none")]
    pub baremetal_s: Option<crate::models::SpotPricesPerBaremetal>,
    #[serde(rename = "baremetal_0", skip_serializing_if = "Option::is_none")]
    pub baremetal_0: Option<crate::models::SpotPricesPerBaremetal>,
}

impl SpotPricesPerFacility {
    pub fn new() -> SpotPricesPerFacility {
        SpotPricesPerFacility {
            baremetal_2a: None,
            baremetal_2a2: None,
            baremetal_1: None,
            baremetal_3: None,
            c2_medium_x86: None,
            baremetal_2: None,
            m2_xlarge_x86: None,
            baremetal_s: None,
            baremetal_0: None,
        }
    }
}


