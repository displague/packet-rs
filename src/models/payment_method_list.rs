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
pub struct PaymentMethodList {
    #[serde(rename = "payment_methods", skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<crate::models::PaymentMethod>>,
}

impl PaymentMethodList {
    pub fn new() -> PaymentMethodList {
        PaymentMethodList {
            payment_methods: None,
        }
    }
}


