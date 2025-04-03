/*
 * Appointments Backend
 *
 * # API for appointment scheduling related data  This is the API of the service at P7M that manages the scheduling and management of appointments. It is used by the booking widget (see the **WidgetApi** tag) with functions that are public and don't require the user to be authenticated.  For endpoints in other tags the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. When using the API you typically get this token by authenticating first with OAuth 2.0.  When you are trying this API using the Swagger interface, you need to click the `Authorize` button and then again the Authorize button in the pop-up that gets opened.
 *
 * The version of the OpenAPI document: 0.13.1
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reservation {
    #[serde(rename = "reservationId")]
    pub reservation_id: uuid::Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    #[serde(rename = "workerId")]
    pub worker_id: uuid::Uuid,
    #[serde(rename = "serviceId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "type")]
    pub r#type: models::ReservationType,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "patientName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub patient_name: Option<Option<String>>,
    #[serde(rename = "patientPhone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub patient_phone: Option<Option<String>>,
    #[serde(rename = "patientMail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub patient_mail: Option<Option<String>>,
    #[serde(rename = "patientBirthday", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub patient_birthday: Option<Option<String>>,
    #[serde(rename = "reservedAt")]
    pub reserved_at: String,
    #[serde(rename = "confirmedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub confirmed_at: Option<Option<String>>,
    #[serde(rename = "confirmationPin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub confirmation_pin: Option<Option<String>>,
    #[serde(rename = "confirmationSent")]
    pub confirmation_sent: bool,
    #[serde(rename = "reminderSent")]
    pub reminder_sent: bool,
    #[serde(rename = "clientNotified")]
    pub client_notified: bool,
    #[serde(rename = "tags")]
    pub tags: Vec<uuid::Uuid>,
    #[serde(rename = "answers")]
    pub answers: String,
    #[serde(rename = "answerShorthands")]
    pub answer_shorthands: String,
    #[serde(rename = "source")]
    pub source: models::SourceMedium,
    #[serde(rename = "notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    #[serde(rename = "sendSmsMessages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub send_sms_messages: Option<Option<bool>>,
    #[serde(rename = "sendMailMessages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub send_mail_messages: Option<Option<bool>>,
}

impl Reservation {
    pub fn new(reservation_id: uuid::Uuid, tenant_id: uuid::Uuid, worker_id: uuid::Uuid, r#type: models::ReservationType, start_time: String, end_time: String, description: String, reserved_at: String, confirmation_sent: bool, reminder_sent: bool, client_notified: bool, tags: Vec<uuid::Uuid>, answers: String, answer_shorthands: String, source: models::SourceMedium) -> Reservation {
        Reservation {
            reservation_id,
            tenant_id,
            worker_id,
            service_id: None,
            r#type,
            start_time,
            end_time,
            description,
            patient_name: None,
            patient_phone: None,
            patient_mail: None,
            patient_birthday: None,
            reserved_at,
            confirmed_at: None,
            confirmation_pin: None,
            confirmation_sent,
            reminder_sent,
            client_notified,
            tags,
            answers,
            answer_shorthands,
            source,
            notes: None,
            send_sms_messages: None,
            send_mail_messages: None,
        }
    }
}

