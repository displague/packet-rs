/*
 * Packet API
 *
 * This is the API for Packet. Interact with your devices, user account, and projects.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@packet.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;

use std::option::Option;

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `find_facilities`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindFacilitiesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_facilities_by_organization`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindFacilitiesByOrganizationError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_facilities_by_project`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindFacilitiesByProjectError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


    pub async fn find_facilities(configuration: &configuration::Configuration, ) -> Result<crate::models::FacilityList, Error<FindFacilitiesError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/facilities", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("X-Auth-Token", val);
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<FindFacilitiesError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_facilities_by_organization(configuration: &configuration::Configuration, id: &str, include: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::FacilityList, Error<FindFacilitiesByOrganizationError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/organizations/{id}/facilities", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = include {
            req_builder = req_builder.query(&[("include", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = per_page {
            req_builder = req_builder.query(&[("per_page", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("X-Auth-Token", val);
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<FindFacilitiesByOrganizationError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_facilities_by_project(configuration: &configuration::Configuration, id: &str, include: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::FacilityList, Error<FindFacilitiesByProjectError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/projects/{id}/facilities", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = include {
            req_builder = req_builder.query(&[("include", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = per_page {
            req_builder = req_builder.query(&[("per_page", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("X-Auth-Token", val);
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<FindFacilitiesByProjectError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }
