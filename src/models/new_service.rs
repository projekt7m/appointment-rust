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
pub struct NewService {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expenditureMinutes")]
    pub expenditure_minutes: i32,
    #[serde(rename = "defaultTags")]
    pub default_tags: Vec<uuid::Uuid>,
    #[serde(rename = "questions")]
    pub questions: Vec<models::ExtraQuestion>,
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
}

impl NewService {
    pub fn new(name: String, expenditure_minutes: i32, default_tags: Vec<uuid::Uuid>, questions: Vec<models::ExtraQuestion>) -> NewService {
        NewService {
            name,
            expenditure_minutes,
            default_tags,
            questions,
            color: None,
        }
    }
}

