/*
 * Appointments Backend
 *
 * # API for appointment scheduling related data  This is the API of the service at P7M that manages the scheduling and management of appointments. It is used by the booking widget (see the **WidgetApi** tag) with functions that are public and don't require the user to be authenticated.  For endpoints in other tags the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. When using the API you typically get this token by authenticating first with OAuth 2.0.  When you are trying this API using the Swagger interface, you need to click the `Authorize` button and then again the Authorize button in the pop-up that gets opened.
 *
 * The version of the OpenAPI document: 0.13.1
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`delete_service_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServiceByIdError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_service_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceByIdError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_services`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServicesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_service_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutServiceByIdError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}


pub async fn delete_service_by_id(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<DeleteServiceByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<DeleteServiceByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_service_by_id(configuration: &configuration::Configuration, id: &str) -> Result<models::Service, Error<GetServiceByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_content = local_var_resp.text().await?;
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<GetServiceByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// A service is an action that can be selected and booked by a patient
pub async fn get_services(configuration: &configuration::Configuration, ) -> Result<models::ListWrapperService, Error<GetServicesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_content = local_var_resp.text().await?;
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<GetServicesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_service(configuration: &configuration::Configuration, new_service: models::NewService) -> Result<models::Service, Error<PostServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&new_service);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_content = local_var_resp.text().await?;
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<PostServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn put_service_by_id(configuration: &configuration::Configuration, id: &str, new_service: models::NewService) -> Result<models::Service, Error<PutServiceByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&new_service);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_content = local_var_resp.text().await?;
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<PutServiceByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

