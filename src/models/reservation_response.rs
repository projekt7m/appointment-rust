/*
 * Appointments Backend
 *
 * # API for appointment scheduling related data  This is the API of the service at P7M that manages the scheduling and management of appointments. It is used by the booking widget (see the **WidgetApi** tag) with functions that are public and don't require the user to be authenticated.  For the endpoint in the other tags, the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. If your interacting with this API using the Swagger interface, you need to set the JWT token by clicking on the **Authorize** button on the right side of the header. As the value don't forget that the Authorization header starts with the fixed value `Bearer` followed by a space followed by the actual JWT token value.  If anything is unclear or you found a bug in the documentation, please contact <tech@p7m.de>. 
 *
 * The version of the OpenAPI document: 0.11.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReservationResponse {
    #[serde(rename = "reservation_id", skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "retry_other_time", skip_serializing_if = "Option::is_none")]
    pub retry_other_time: Option<bool>,
    #[serde(rename = "medium", skip_serializing_if = "Option::is_none")]
    pub medium: Option<Medium>,
}

impl ReservationResponse {
    pub fn new(success: bool) -> ReservationResponse {
        ReservationResponse {
            reservation_id: None,
            success,
            retry_other_time: None,
            medium: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Medium {
    #[serde(rename = "SMS")]
    SMS,
    #[serde(rename = "VOICE_CALL")]
    VOICECALL,
    #[serde(rename = "EMAIL")]
    EMAIL,
}

impl Default for Medium {
    fn default() -> Medium {
        Self::SMS
    }
}
