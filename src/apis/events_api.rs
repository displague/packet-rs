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


/// struct for typed errors of method `find_device_events`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindDeviceEventsError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_event_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindEventByIdError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_events`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindEventsError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_project_events`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindProjectEventsError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_volume_events`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindVolumeEventsError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


    pub async fn find_device_events(configuration: &configuration::Configuration, id: &str, include: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::EventList, Error<FindDeviceEventsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/devices/{id}/events", configuration.base_path, id=id);
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
            let entity: Option<FindDeviceEventsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_event_by_id(configuration: &configuration::Configuration, id: &str, include: Option<&str>) -> Result<crate::models::Event, Error<FindEventByIdError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/events/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = include {
            req_builder = req_builder.query(&[("include", &s.to_string())]);
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
            let entity: Option<FindEventByIdError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_events(configuration: &configuration::Configuration, include: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::EventList, Error<FindEventsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/events", configuration.base_path);
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
            let entity: Option<FindEventsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_project_events(configuration: &configuration::Configuration, id: &str, include: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::EventList, Error<FindProjectEventsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/projects/{id}/events", configuration.base_path, id=id);
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
            let entity: Option<FindProjectEventsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_volume_events(configuration: &configuration::Configuration, id: &str, include: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::EventList, Error<FindVolumeEventsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/volumes/{id}/events", configuration.base_path, id=id);
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
            let entity: Option<FindVolumeEventsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }
