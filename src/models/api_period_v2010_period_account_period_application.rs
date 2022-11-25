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
pub struct ApiPeriodV2010PeriodAccountPeriodApplication {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The API version used to start a new TwiML session
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// The URL to send message status information to your application
    #[serde(rename = "message_status_callback", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message_status_callback: Option<Option<String>>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The HTTP method used with sms_fallback_url
    #[serde(rename = "sms_fallback_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_fallback_method: Option<Option<SmsFallbackMethod>>,
    /// The URL that we call when an error occurs while retrieving or executing the TwiML
    #[serde(rename = "sms_fallback_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_fallback_url: Option<Option<String>>,
    /// The HTTP method to use with sms_url
    #[serde(rename = "sms_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_method: Option<Option<SmsMethod>>,
    /// The URL to send status information to your application
    #[serde(rename = "sms_status_callback", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_status_callback: Option<Option<String>>,
    /// The URL we call when the phone number receives an incoming SMS message
    #[serde(rename = "sms_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_url: Option<Option<String>>,
    /// The URL to send status information to your application
    #[serde(rename = "status_callback", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<Option<String>>,
    /// The HTTP method we use to call status_callback
    #[serde(rename = "status_callback_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_callback_method: Option<Option<StatusCallbackMethod>>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// Whether to lookup the caller's name
    #[serde(rename = "voice_caller_id_lookup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_caller_id_lookup: Option<Option<bool>>,
    /// The HTTP method used with voice_fallback_url
    #[serde(rename = "voice_fallback_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_fallback_method: Option<Option<VoiceFallbackMethod>>,
    /// The URL we call when a TwiML error occurs
    #[serde(rename = "voice_fallback_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_fallback_url: Option<Option<String>>,
    /// The HTTP method used with the voice_url
    #[serde(rename = "voice_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_method: Option<Option<VoiceMethod>>,
    /// The URL we call when the phone number receives a call
    #[serde(rename = "voice_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_url: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodApplication {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodApplication {
        ApiPeriodV2010PeriodAccountPeriodApplication {
            account_sid: None,
            api_version: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            message_status_callback: None,
            sid: None,
            sms_fallback_method: None,
            sms_fallback_url: None,
            sms_method: None,
            sms_status_callback: None,
            sms_url: None,
            status_callback: None,
            status_callback_method: None,
            uri: None,
            voice_caller_id_lookup: None,
            voice_fallback_method: None,
            voice_fallback_url: None,
            voice_method: None,
            voice_url: None,
        }
    }
}

/// The HTTP method used with sms_fallback_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsFallbackMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for SmsFallbackMethod {
    fn default() -> SmsFallbackMethod {
        Self::Head
    }
}
/// The HTTP method to use with sms_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for SmsMethod {
    fn default() -> SmsMethod {
        Self::Head
    }
}
/// The HTTP method we use to call status_callback
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCallbackMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for StatusCallbackMethod {
    fn default() -> StatusCallbackMethod {
        Self::Head
    }
}
/// The HTTP method used with voice_fallback_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceFallbackMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for VoiceFallbackMethod {
    fn default() -> VoiceFallbackMethod {
        Self::Head
    }
}
/// The HTTP method used with the voice_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for VoiceMethod {
    fn default() -> VoiceMethod {
        Self::Head
    }
}

