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
pub struct ApiPeriodV2010PeriodAccountPeriodMessage {
    /// The message text
    #[serde(rename = "body", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub body: Option<Option<String>>,
    /// The number of messages used to deliver the message body
    #[serde(rename = "num_segments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_segments: Option<Option<String>>,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<crate::models::MessageEnumDirection>,
    /// The phone number that initiated the message
    #[serde(rename = "from", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from: Option<Option<String>>,
    /// The phone number that received the message
    #[serde(rename = "to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The amount billed for the message
    #[serde(rename = "price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price: Option<Option<String>>,
    /// The description of the error_code
    #[serde(rename = "error_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The number of media files associated with the message
    #[serde(rename = "num_media", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_media: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::MessageEnumStatus>,
    /// The SID of the Messaging Service used with the message.
    #[serde(rename = "messaging_service_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messaging_service_sid: Option<Option<String>>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT when the message was sent
    #[serde(rename = "date_sent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_sent: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The error code associated with the message
    #[serde(rename = "error_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<i32>>,
    /// The currency in which price is measured
    #[serde(rename = "price_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<Option<String>>,
    /// The API version used to process the message
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// A list of related resources identified by their relative URIs
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
}

impl ApiPeriodV2010PeriodAccountPeriodMessage {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodMessage {
        ApiPeriodV2010PeriodAccountPeriodMessage {
            body: None,
            num_segments: None,
            direction: None,
            from: None,
            to: None,
            date_updated: None,
            price: None,
            error_message: None,
            uri: None,
            account_sid: None,
            num_media: None,
            status: None,
            messaging_service_sid: None,
            sid: None,
            date_sent: None,
            date_created: None,
            error_code: None,
            price_unit: None,
            api_version: None,
            subresource_uris: None,
        }
    }
}


