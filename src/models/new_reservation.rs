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
pub struct NewReservation {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "workerId")]
    pub worker_id: String,
    #[serde(rename = "serviceId", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "patientName", skip_serializing_if = "Option::is_none")]
    pub patient_name: Option<String>,
    #[serde(rename = "patientPhone", skip_serializing_if = "Option::is_none")]
    pub patient_phone: Option<String>,
    #[serde(rename = "patientMail", skip_serializing_if = "Option::is_none")]
    pub patient_mail: Option<String>,
    #[serde(rename = "reservedAt", skip_serializing_if = "Option::is_none")]
    pub reserved_at: Option<String>,
    #[serde(rename = "confirmedAt", skip_serializing_if = "Option::is_none")]
    pub confirmed_at: Option<String>,
    #[serde(rename = "confirmationPin", skip_serializing_if = "Option::is_none")]
    pub confirmation_pin: Option<String>,
    #[serde(rename = "confirmationSent")]
    pub confirmation_sent: bool,
    #[serde(rename = "reminderSent")]
    pub reminder_sent: bool,
    #[serde(rename = "clientNotified")]
    pub client_notified: bool,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "answers")]
    pub answers: String,
    #[serde(rename = "answerShorthands")]
    pub answer_shorthands: String,
    #[serde(rename = "source")]
    pub source: Source,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "sendSmsMessages", skip_serializing_if = "Option::is_none")]
    pub send_sms_messages: Option<bool>,
    #[serde(rename = "sendMailMessages", skip_serializing_if = "Option::is_none")]
    pub send_mail_messages: Option<bool>,
}

impl NewReservation {
    pub fn new(tenant_id: String, worker_id: String, _type: Type, start_time: String, end_time: String, description: String, confirmation_sent: bool, reminder_sent: bool, client_notified: bool, tags: Vec<String>, answers: String, answer_shorthands: String, source: Source) -> NewReservation {
        NewReservation {
            tenant_id,
            worker_id,
            service_id: None,
            _type,
            start_time,
            end_time,
            description,
            patient_name: None,
            patient_phone: None,
            patient_mail: None,
            reserved_at: None,
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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PATIENT")]
    PATIENT,
    #[serde(rename = "BLOCKED")]
    BLOCKED,
}

impl Default for Type {
    fn default() -> Type {
        Self::PATIENT
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "WEB")]
    WEB,
    #[serde(rename = "PHONE")]
    PHONE,
    #[serde(rename = "STAFF")]
    STAFF,
}

impl Default for Source {
    fn default() -> Source {
        Self::WEB
    }
}

