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
pub struct BookingTenant {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "reservationLimitMode")]
    pub reservation_limit_mode: ReservationLimitMode,
    #[serde(rename = "reservationLimitDate", skip_serializing_if = "Option::is_none")]
    pub reservation_limit_date: Option<String>,
    #[serde(rename = "reservationLimitDays", skip_serializing_if = "Option::is_none")]
    pub reservation_limit_days: Option<i32>,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "phone")]
    pub phone: String,
    #[serde(rename = "fax")]
    pub fax: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "emailSignature")]
    pub email_signature: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "emailInternal")]
    pub email_internal: String,
    #[serde(rename = "calendarExtraInfo")]
    pub calendar_extra_info: String,
    #[serde(rename = "requiredConditions")]
    pub required_conditions: Vec<String>,
}

impl BookingTenant {
    pub fn new(tenant_id: String, timezone: String, reservation_limit_mode: ReservationLimitMode, address: String, phone: String, fax: String, email: String, email_signature: String, name: String, email_internal: String, calendar_extra_info: String, required_conditions: Vec<String>) -> BookingTenant {
        BookingTenant {
            tenant_id,
            timezone,
            reservation_limit_mode,
            reservation_limit_date: None,
            reservation_limit_days: None,
            address,
            phone,
            fax,
            email,
            email_signature,
            name,
            email_internal,
            calendar_extra_info,
            required_conditions,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReservationLimitMode {
    #[serde(rename = "BY_DATE")]
    DATE,
    #[serde(rename = "BY_DAYS")]
    DAYS,
}

impl Default for ReservationLimitMode {
    fn default() -> ReservationLimitMode {
        Self::DATE
    }
}

