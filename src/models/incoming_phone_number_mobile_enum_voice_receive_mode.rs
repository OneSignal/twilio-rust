/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.37.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncomingPhoneNumberMobileEnumVoiceReceiveMode {
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "fax")]
    Fax,

}

impl ToString for IncomingPhoneNumberMobileEnumVoiceReceiveMode {
    fn to_string(&self) -> String {
        match self {
            Self::Voice => String::from("voice"),
            Self::Fax => String::from("fax"),
        }
    }
}

impl Default for IncomingPhoneNumberMobileEnumVoiceReceiveMode {
    fn default() -> IncomingPhoneNumberMobileEnumVoiceReceiveMode {
        Self::Voice
    }
}




