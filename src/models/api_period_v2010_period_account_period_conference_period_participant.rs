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
pub struct ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The SID of the Call the resource is associated with
    #[serde(rename = "call_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<Option<String>>,
    /// The label of this participant
    #[serde(rename = "label", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub label: Option<Option<String>>,
    /// The SID of the participant who is being `coached`
    #[serde(rename = "call_sid_to_coach", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_sid_to_coach: Option<Option<String>>,
    /// Indicates if the participant changed to coach
    #[serde(rename = "coaching", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub coaching: Option<Option<bool>>,
    /// The SID of the conference the participant is in
    #[serde(rename = "conference_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub conference_sid: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// Whether the conference ends when the participant leaves
    #[serde(rename = "end_conference_on_exit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_conference_on_exit: Option<Option<bool>>,
    /// Whether the participant is muted
    #[serde(rename = "muted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub muted: Option<Option<bool>>,
    /// Whether the participant is on hold
    #[serde(rename = "hold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hold: Option<Option<bool>>,
    /// Whether the conference starts when the participant joins the conference
    #[serde(rename = "start_conference_on_enter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_conference_on_enter: Option<Option<bool>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ParticipantEnumStatus>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant {
        ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant {
            account_sid: None,
            call_sid: None,
            label: None,
            call_sid_to_coach: None,
            coaching: None,
            conference_sid: None,
            date_created: None,
            date_updated: None,
            end_conference_on_exit: None,
            muted: None,
            hold: None,
            start_conference_on_enter: None,
            status: None,
            uri: None,
        }
    }
}

