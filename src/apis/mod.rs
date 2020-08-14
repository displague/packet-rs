use reqwest;
use serde_json;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod bgp_api;
pub mod batches_api;
pub mod capacity_api;
pub mod devices_api;
pub mod emails_api;
pub mod events_api;
pub mod facilities_api;
pub mod hardware_reservations_api;
pub mod ip_addresses_api;
pub mod incidents_api;
pub mod internet_gateways_api;
pub mod invitations_api;
pub mod licenses_api;
pub mod market_api;
pub mod memberships_api;
pub mod notifications_api;
pub mod operating_system_versions_api;
pub mod operating_systems_api;
pub mod organizations_api;
pub mod otps_api;
pub mod password_reset_tokens_api;
pub mod payment_methods_api;
pub mod plans_api;
pub mod ports_api;
pub mod projects_api;
pub mod ssh_keys_api;
pub mod spot_market_request_api;
pub mod transfer_requests_api;
pub mod two_factor_auth_api;
pub mod usages_api;
pub mod user_verification_tokens_api;
pub mod userdata_api;
pub mod users_api;
pub mod vla_ns_api;
pub mod vpn_api;
pub mod volumes_api;

pub mod configuration;
