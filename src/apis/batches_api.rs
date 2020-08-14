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


/// struct for typed errors of method `create_device_batch`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDeviceBatchError {
    Status401(),
    Status403(),
    Status404(),
    Status422(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_batch`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBatchError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_batch_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindBatchByIdError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_batches_by_project`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindBatchesByProjectError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


    pub async fn create_device_batch(configuration: &configuration::Configuration, id: &str, batch: crate::models::InstancesBatchCreateInput) -> Result<crate::models::BatchesList, Error<CreateDeviceBatchError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/projects/{id}/devices/batch", configuration.base_path, id=id);
        let mut req_builder = client.post(uri_str.as_str());

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
        req_builder = req_builder.json(&batch);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<CreateDeviceBatchError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn delete_batch(configuration: &configuration::Configuration, id: &str, remove_associated_instances: bool) -> Result<(), Error<DeleteBatchError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/batches/{id}", configuration.base_path, id=id);
        let mut req_builder = client.delete(uri_str.as_str());

        req_builder = req_builder.query(&[("remove_associated_instances", &remove_associated_instances.to_string())]);
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
            Ok(())
        } else {
            let entity: Option<DeleteBatchError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_batch_by_id(configuration: &configuration::Configuration, id: &str, include: Option<&str>) -> Result<crate::models::Batch, Error<FindBatchByIdError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/batches/{id}", configuration.base_path, id=id);
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
            let entity: Option<FindBatchByIdError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_batches_by_project(configuration: &configuration::Configuration, id: &str, include: Option<&str>) -> Result<crate::models::BatchesList, Error<FindBatchesByProjectError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/projects/{id}/batches", configuration.base_path, id=id);
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
            let entity: Option<FindBatchesByProjectError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }
