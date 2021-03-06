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
pub struct SpotMarketPricesList {
    #[serde(rename = "spot_market_prices", skip_serializing_if = "Option::is_none")]
    pub spot_market_prices: Option<crate::models::SpotPricesReport>,
}

impl SpotMarketPricesList {
    pub fn new() -> SpotMarketPricesList {
        SpotMarketPricesList {
            spot_market_prices: None,
        }
    }
}


