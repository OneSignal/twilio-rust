/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.37.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage {
    /// Account SID.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// Call SID.
    #[serde(rename = "call_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<Option<String>>,
    /// User Defined Message SID.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The date this User Defined Message was created.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage {
        ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage {
            account_sid: None,
            call_sid: None,
            sid: None,
            date_created: None,
        }
    }
}

