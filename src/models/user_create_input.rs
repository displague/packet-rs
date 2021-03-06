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
pub struct UserCreateInput {
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "company_url", skip_serializing_if = "Option::is_none")]
    pub company_url: Option<String>,
    #[serde(rename = "verified_at", skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
    #[serde(rename = "social_accounts", skip_serializing_if = "Option::is_none")]
    pub social_accounts: Option<serde_json::Value>,
    #[serde(rename = "two_factor_auth", skip_serializing_if = "Option::is_none")]
    pub two_factor_auth: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<std::path::PathBuf>,
    #[serde(rename = "emails")]
    pub emails: Vec<crate::models::EmailInput>,
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "customdata", skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,
}

impl UserCreateInput {
    pub fn new(first_name: String, last_name: String, emails: Vec<crate::models::EmailInput>) -> UserCreateInput {
        UserCreateInput {
            first_name,
            last_name,
            phone_number: None,
            timezone: None,
            password: None,
            level: None,
            title: None,
            company_name: None,
            company_url: None,
            verified_at: None,
            social_accounts: None,
            two_factor_auth: None,
            avatar: None,
            emails,
            locked: None,
            customdata: None,
        }
    }
}


