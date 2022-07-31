/*
 * Appointments Backend
 *
 * # API for appointment scheduling related data  This is the API of the service at P7M that manages the scheduling and management of appointments. It is used by the booking widget (see the **WidgetApi** tag) with functions that are public and don't require the user to be authenticated.  For the endpoint in the other tags, the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. If your interacting with this API using the Swagger interface, you need to set the JWT token by clicking on the **Authorize** button on the right side of the header. As the value don't forget that the Authorization header starts with the fixed value `Bearer` followed by a space followed by the actual JWT token value.  If anything is unclear or you found a bug in the documentation, please contact <tech@p7m.de>. 
 *
 * The version of the OpenAPI document: 0.11.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`booking_config_tid_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookingConfigTidGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`booking_reserve_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookingReservePostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`booking_reserve_rid_confirmation_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookingReserveRidConfirmationPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`booking_reserve_rid_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookingReserveRidPutError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`booking_schedule_tid_sid_wid_date_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookingScheduleTidSidWidDateGetError {
    UnknownValue(serde_json::Value),
}


/// # Provides basic configuration information for the widget of the specified tenant.  It is the entry point for the widget. The widget only needs to know in the page of which tenant it is used, then it sends out a request to this endpoint to know its basic configuration, services and workers it can offer to the user. 
pub async fn booking_config_tid_get(configuration: &configuration::Configuration, tid: &str) -> Result<crate::models::TenantConfig, Error<BookingConfigTidGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/booking/config/{tid}", local_var_configuration.base_path, tid=crate::apis::urlencode(tid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BookingConfigTidGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # Reserve a time slot for a user  After the user has selected a time slot, that it wants to use, the widget will use this endpoint to reserve the given time slot. The server will check, whether the slot is still available, send out the verification code to the user by SMS or phone call, and return to the caller whether the slot could be reserved.  For final booking of the time slot, the user has to give the widget the PIN it got sent, and the widget has to send the PIN with another request to the server, to finally book the reservation it made. Reservations, that do not get booked, will be canceled after a configured amount of time.  **Note:** This endpoint is especially purposed for the self-service of the user using the booking widget, therefore a reservation created here will be stored with a source type of `WEB` 
pub async fn booking_reserve_post(configuration: &configuration::Configuration, reservation_request: crate::models::ReservationRequest) -> Result<crate::models::ReservationResponse, Error<BookingReservePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/booking/reserve", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&reservation_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BookingReservePostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # Confirm a reservation with the PIN sent to the user  After the user has entered the PIN he received in the Widget, the widget has to send the PIN to the server using this function. This on the one hand will confirm the reservation to the server, and on the other hand will tell the widget whether the entered PIN has been correct. 
pub async fn booking_reserve_rid_confirmation_post(configuration: &configuration::Configuration, rid: &str, reservation_confirmation_request: crate::models::ReservationConfirmationRequest) -> Result<crate::models::ReservationConfirmationResponse, Error<BookingReserveRidConfirmationPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/booking/reserve/{rid}/confirmation", local_var_configuration.base_path, rid=crate::apis::urlencode(rid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&reservation_confirmation_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BookingReserveRidConfirmationPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # Update a reservation  This has two purposes: - it can be used to change the phone number or other data used when a reservation has been created (e.g. because   a user detected, that he made a typo) - after updating (even with the same data as before) the verification PIN is resent, so it can be used if the   user did not receive the PIN and the PIN has to be sent another time  Note: updates to a reservation are only possible as long as the reservation has not been confirmed by the user. Once the user confirms the reservation by entering the PIN he has received from the system, the reservation is fixed. 
pub async fn booking_reserve_rid_put(configuration: &configuration::Configuration, rid: &str, reservation_request: crate::models::ReservationRequest) -> Result<crate::models::ReservationResponse, Error<BookingReserveRidPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/booking/reserve/{rid}", local_var_configuration.base_path, rid=crate::apis::urlencode(rid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&reservation_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BookingReserveRidPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # Returns the available times at which a service can be booked.  After the user has selected a service and a worker, the Widget will use this endpoint to request the time slots that it can provide to the user. Time slots are requested by date. 
pub async fn booking_schedule_tid_sid_wid_date_get(configuration: &configuration::Configuration, tid: &str, sid: &str, wid: &str, date: String) -> Result<crate::models::Availabilities, Error<BookingScheduleTidSidWidDateGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/booking/schedule/{tid}/{sid}/{wid}/{date}", local_var_configuration.base_path, tid=crate::apis::urlencode(tid), sid=crate::apis::urlencode(sid), wid=crate::apis::urlencode(wid), date=date);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BookingScheduleTidSidWidDateGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

