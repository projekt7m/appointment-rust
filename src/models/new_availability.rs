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
pub struct NewAvailability {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "workerId")]
    pub worker_id: String,
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: DayOfWeek,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "services")]
    pub services: Vec<String>,
    #[serde(rename = "notBefore", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(rename = "notAfter", skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
}

impl NewAvailability {
    pub fn new(tenant_id: String, worker_id: String, day_of_week: DayOfWeek, start_time: String, end_time: String, services: Vec<String>) -> NewAvailability {
        NewAvailability {
            tenant_id,
            worker_id,
            day_of_week,
            start_time,
            end_time,
            services,
            not_before: None,
            not_after: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DayOfWeek {
    #[serde(rename = "MONDAY")]
    MONDAY,
    #[serde(rename = "TUESDAY")]
    TUESDAY,
    #[serde(rename = "WEDNESDAY")]
    WEDNESDAY,
    #[serde(rename = "THURSDAY")]
    THURSDAY,
    #[serde(rename = "FRIDAY")]
    FRIDAY,
    #[serde(rename = "SATURDAY")]
    SATURDAY,
    #[serde(rename = "SUNDAY")]
    SUNDAY,
}

impl Default for DayOfWeek {
    fn default() -> DayOfWeek {
        Self::MONDAY
    }
}
