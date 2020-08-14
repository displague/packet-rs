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
pub struct CapacityPerBaremetal {
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "available_servers", skip_serializing_if = "Option::is_none")]
    pub available_servers: Option<i32>,
    #[serde(rename = "total_servers", skip_serializing_if = "Option::is_none")]
    pub total_servers: Option<i32>,
    #[serde(rename = "market_buffer_percentage", skip_serializing_if = "Option::is_none")]
    pub market_buffer_percentage: Option<i32>,
    #[serde(rename = "market_floor_price", skip_serializing_if = "Option::is_none")]
    pub market_floor_price: Option<f32>,
}

impl CapacityPerBaremetal {
    pub fn new() -> CapacityPerBaremetal {
        CapacityPerBaremetal {
            level: None,
            available_servers: None,
            total_servers: None,
            market_buffer_percentage: None,
            market_floor_price: None,
        }
    }
}


