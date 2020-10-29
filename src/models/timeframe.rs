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
pub struct Timeframe {
    #[serde(rename = "started_at")]
    pub started_at: String,
    #[serde(rename = "ended_at")]
    pub ended_at: String,
}

impl Timeframe {
    pub fn new(started_at: String, ended_at: String) -> Timeframe {
        Timeframe {
            started_at,
            ended_at,
        }
    }
}


