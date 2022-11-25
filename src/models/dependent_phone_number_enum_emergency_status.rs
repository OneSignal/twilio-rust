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
pub enum DependentPhoneNumberEnumEmergencyStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,

}

impl ToString for DependentPhoneNumberEnumEmergencyStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("Active"),
            Self::Inactive => String::from("Inactive"),
        }
    }
}

impl Default for DependentPhoneNumberEnumEmergencyStatus {
    fn default() -> DependentPhoneNumberEnumEmergencyStatus {
        Self::Active
    }
}




