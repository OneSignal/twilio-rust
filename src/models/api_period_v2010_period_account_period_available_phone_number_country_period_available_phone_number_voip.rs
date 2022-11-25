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
pub struct ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberVoip {
    /// A formatted version of the phone number
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// The phone number in E.164 format
    #[serde(rename = "phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    /// The LATA of this phone number
    #[serde(rename = "lata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lata: Option<Option<String>>,
    /// The locality or city of this phone number's location
    #[serde(rename = "locality", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub locality: Option<Option<String>>,
    /// The rate center of this phone number
    #[serde(rename = "rate_center", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rate_center: Option<Option<String>>,
    /// The latitude of this phone number's location
    #[serde(rename = "latitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Option<f32>>,
    /// The longitude of this phone number's location
    #[serde(rename = "longitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Option<f32>>,
    /// The two-letter state or province abbreviation of this phone number's location
    #[serde(rename = "region", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub region: Option<Option<String>>,
    /// The postal or ZIP code of this phone number's location
    #[serde(rename = "postal_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Option<String>>,
    /// The ISO country code of this phone number
    #[serde(rename = "iso_country", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<Option<String>>,
    /// The type of Address resource the phone number requires
    #[serde(rename = "address_requirements", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_requirements: Option<Option<String>>,
    /// Whether the phone number is new to the Twilio platform
    #[serde(rename = "beta", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub beta: Option<Option<bool>>,
    #[serde(rename = "capabilities", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Option<Box<crate::models::ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities>>>,
}

impl ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberVoip {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberVoip {
        ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberVoip {
            friendly_name: None,
            phone_number: None,
            lata: None,
            locality: None,
            rate_center: None,
            latitude: None,
            longitude: None,
            region: None,
            postal_code: None,
            iso_country: None,
            address_requirements: None,
            beta: None,
            capabilities: None,
        }
    }
}

