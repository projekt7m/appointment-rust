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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SourceMedium {
    #[serde(rename = "WEB")]
    Web,
    #[serde(rename = "PHONE")]
    Phone,
    #[serde(rename = "STAFF")]
    Staff,

}

impl std::fmt::Display for SourceMedium {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Web => write!(f, "WEB"),
            Self::Phone => write!(f, "PHONE"),
            Self::Staff => write!(f, "STAFF"),
        }
    }
}

impl Default for SourceMedium {
    fn default() -> SourceMedium {
        Self::Web
    }
}

