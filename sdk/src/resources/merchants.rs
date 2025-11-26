// The contents of this file are generated; do not modify them.

use super::common::*;
/// An address somewhere in the world. The address fields used depend on the country conventions. For example, in Great Britain, `city` is `post_town`. In the United States, the top-level administrative unit used in addresses is `state`, whereas in Chile it's `region`.
/// Whether an address is valid or not depends on whether the locally required fields are present. Fields not supported in a country will be ignored.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<Vec<String>>,
    /// The postal code (aka. zip code) of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    pub country: CountryCode,
    /// The city of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The province where the address is located. This may not be relevant in some countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// The region where the address is located. This may not be relevant in some countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// A county is a geographic region of a country used for administrative or other purposes in some nations. Used in countries such as Ireland, Romania, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// In Spain, an autonomous community is the first sub-national level of political and administrative division.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous_community: Option<String>,
    /// A post town is a required part of all postal addresses in the United Kingdom and Ireland, and a basic unit of the postal delivery system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_town: Option<String>,
    /// Most often, a country has a single state, with various administrative divisions. The term "state" is sometimes used to refer to the federated polities that make up the federation. Used in countries such as the United States and Brazil.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Locality level of the address. Used in countries such as Brazil or Chile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighborhood: Option<String>,
    /// In many countries, terms cognate with "commune" are used, referring to the community living in the area and the common interest. Used in countries such as Chile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commune: Option<String>,
    /// A department (French: département, Spanish: departamento) is an administrative or political division in several countries. Used in countries such as Colombia.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// A municipality is usually a single administrative division having corporate status and powers of self-government or jurisdiction as granted by national and regional laws to which it is subordinate. Used in countries such as Colombia.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality: Option<String>,
    /// A district is a type of administrative division that in some countries is managed by the local government. Used in countries such as Portugal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// A US system of postal codes used by the United States Postal Service (USPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// A postal address in Ireland.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eircode: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BaseError {
    /// A unique identifier for the error instance. This can be used to trace the error back to the server logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// A human-readable message describing the error that occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
/// Base schema for a person associated with a merchant. This can be a legal representative, business owner (ultimate beneficial owner), or an officer. A legal representative is the person who registered the merchant with SumUp. They should always have a `user_id`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasePerson {
    /// The unique identifier for the person. This is a [typeid](https://github.com/sumup/typeid).
    pub id: String,
    /// A corresponding identity user ID for the person, if they have a user account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The date of birth of the individual, represented as an ISO 8601:2004 [ISO8601‑2004] YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<crate::datetime::Date>,
    /// The first name(s) of the individual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// The last name(s) of the individual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// Middle name(s) of the End-User. Note that in some cultures, people can have multiple middle names; all can be present, with the names being separated by space characters. Also note that in some cultures, middle names are not used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
    /// A list of roles the person has in the merchant or towards SumUp. A merchant must have at least one person with the relationship `representative`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<Ownership>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// A list of country-specific personal identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<PersonalIdentifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<CountryCode>,
    /// The persons nationality. May be an [ISO3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code, but legacy data may not conform to this standard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// An [ISO3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code representing the country where the person resides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_residence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_status: Option<ChangeStatus>,
}
/// Settings used to apply the Merchant's branding to email receipts, invoices, checkouts, and other products.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Branding {
    /// An icon for the merchant. Must be square.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// A logo for the merchant that will be used in place of the icon and without the merchant's name next to it if there's sufficient space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Data-URL encoded hero image for the merchant business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero: Option<String>,
    /// A hex color value representing the primary branding color of this merchant (your brand color).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    /// A hex color value representing the color of the text displayed on branding color of this merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color_fg: Option<String>,
    /// A hex color value representing the secondary branding color of this merchant (accent color used for buttons).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
    /// A hex color value representing the color of the text displayed on secondary branding color of this merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color_fg: Option<String>,
    /// A hex color value representing the preferred background color of this merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
}
/// Business information about the merchant. This information will be visible to the merchant's customers.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BusinessProfile {
    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The descriptor is the text that your customer sees on their bank account statement.
    /// The more recognisable your descriptor is, the less risk you have of receiving disputes (e.g. chargebacks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_descriptor: Option<String>,
    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// A publicly available email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<Branding>,
}
pub type ChangeStatus = String;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClassicMerchantIdentifiers {
    /// Classic (serial) merchant ID.
    #[deprecated]
    pub id: i64,
}
/// Information about the company or business. This is legal information that is used for verification.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Company {
    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The merchant category code for the account as specified by [ISO18245](https://www.iso.org/standard/33365.html). MCCs are used to classify businesses based on the goods or services they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_type: Option<LegalType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<CompanyIdentifiers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
    /// HTTP(S) URL of the company's website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompanyIdentifier {
    /// The unique reference for the company identifier type as defined in the country SDK.
    #[serde(rename = "ref")]
    pub ref_: String,
    /// The company identifier value.
    pub value: String,
}
pub type CompanyIdentifiers = Vec<CompanyIdentifier>;
pub type CountryCode = String;
/// The category of the error.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorCategoryClient {
    #[serde(rename = "client_error")]
    ClientError,
}
/// The category of the error.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorCategoryServer {
    #[serde(rename = "server_error")]
    ServerError,
}
/// An error code specifying the exact error that occurred.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorCodeInternalServerError {
    #[serde(rename = "internal_error")]
    InternalError,
}
/// An error code specifying the exact error that occurred.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorCodeNotFound {
    #[serde(rename = "not_found")]
    NotFound,
}
pub type LegalType = String;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListPersonsResponseBody {
    pub items: Vec<Person>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Merchant {
    /// Short unique identifier for the merchant.
    pub merchant_code: String,
    /// ID of the organization the merchant belongs to (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// The business type.
    /// * `sole_trader`: The business is run by an self-employed individual.
    /// * `company`: The business is run as a company with one or more shareholders
    /// * `partnership`: The business is run as a company with two or more shareholders that can be also other legal entities
    /// * `non_profit`: The business is run as a nonprofit organization that operates for public or social benefit
    /// * `government_entity`: The business is state owned and operated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,
    pub country: CountryCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile>,
    /// A user-facing small-format logo for use in dashboards and other user-facing applications. For customer-facing branding see `merchant.business_profile.branding`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// A user-facing name of the merchant account for use in dashboards and other user-facing applications. For customer-facing business name see `merchant.business_profile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Three-letter [ISO currency code](https://en.wikipedia.org/wiki/ISO_4217) representing the default currency for the account.
    pub default_currency: String,
    /// Merchant's default locale, represented as a BCP47 [RFC5646](https://datatracker.ietf.org/doc/html/rfc5646) language tag. This is typically an ISO 639-1 Alpha-2 [ISO639‑1](https://www.iso.org/iso-639-language-code) language code in lowercase and an ISO 3166-1 Alpha-2 [ISO3166‑1](https://www.iso.org/iso-3166-country-codes.html) country code in uppercase, separated by a dash. For example, en-US or fr-CA.
    /// In multilingual countries this is the merchant's preferred locale out of those, that are officially spoken in the country. In a countries with a single official language this will match the official language.
    pub default_locale: String,
    /// True if the merchant is a sandbox for testing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic: Option<ClassicMerchantIdentifiers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_status: Option<ChangeStatus>,
    /// The date and time when the resource was created. This is a string as defined in [RFC 3339, section 5.6](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6).
    pub created_at: crate::datetime::DateTime,
    /// The date and time when the resource was last updated. This is a string as defined in [RFC 3339, section 5.6](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6).
    pub updated_at: crate::datetime::DateTime,
}
/// A set of key-value pairs that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
///
/// **Warning**: Updating Meta will overwrite the existing data. Make sure to always include the complete JSON object.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Meta {}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ownership {
    /// The percent of ownership shares held by the person expressed in percent mille (1/100000). Only persons with the relationship `owner` can have ownership.
    pub share: i32,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Person {
    /// The unique identifier for the person. This is a [typeid](https://github.com/sumup/typeid).
    pub id: String,
    /// A corresponding identity user ID for the person, if they have a user account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The date of birth of the individual, represented as an ISO 8601:2004 [ISO8601‑2004] YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<crate::datetime::Date>,
    /// The first name(s) of the individual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// The last name(s) of the individual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// Middle name(s) of the End-User. Note that in some cultures, people can have multiple middle names; all can be present, with the names being separated by space characters. Also note that in some cultures, middle names are not used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
    /// A list of roles the person has in the merchant or towards SumUp. A merchant must have at least one person with the relationship `representative`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<Ownership>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// A list of country-specific personal identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<PersonalIdentifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<CountryCode>,
    /// The persons nationality. May be an [ISO3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code, but legacy data may not conform to this standard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// An [ISO3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code representing the country where the person resides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_residence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_status: Option<ChangeStatus>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersonalIdentifier {
    /// The unique reference for the personal identifier type.
    #[serde(rename = "ref")]
    pub ref_: String,
    /// The company identifier value.
    pub value: String,
}
pub type PhoneNumber = String;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Timestamps {
    /// The date and time when the resource was created. This is a string as defined in [RFC 3339, section 5.6](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6).
    pub created_at: crate::datetime::DateTime,
    /// The date and time when the resource was last updated. This is a string as defined in [RFC 3339, section 5.6](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6).
    pub updated_at: crate::datetime::DateTime,
}
pub type Version = String;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetMerchantParams {
    /// The version of the resource. At the moment, the only supported value is `latest`. When provided and the requested resource's `change_status` is pending, the resource will be returned with all pending changes applied. When no changes are pending the resource is returned as is. The `change_status` in the response body will reflect the current state of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListPersonsParams {
    /// The version of the resource. At the moment, the only supported value is `latest`. When provided and the requested resource's `change_status` is pending, the resource will be returned with all pending changes applied. When no changes are pending the resource is returned as is. The `change_status` in the response body will reflect the current state of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetPersonParams {
    /// The version of the resource. At the moment, the only supported value is `latest`. When provided and the requested resource's `change_status` is pending, the resource will be returned with all pending changes applied. When no changes are pending the resource is returned as is. The `change_status` in the response body will reflect the current state of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
use crate::client::Client;
#[derive(Debug)]
pub enum GetMerchantErrorBody {
    NotFound,
}
#[derive(Debug)]
pub enum ListPersonsErrorBody {
    NotFound,
    InternalServerError,
}
#[derive(Debug)]
pub enum GetPersonErrorBody {
    NotFound,
    InternalServerError,
}
///Client for the Merchants API endpoints.
#[derive(Debug)]
pub struct MerchantsClient<'a> {
    client: &'a Client,
}
impl<'a> MerchantsClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// Retrieve a Merchant
    ///
    /// Retrieve a merchant.
    pub async fn get(
        &self,
        merchant_code: impl Into<String>,
        params: GetMerchantParams,
    ) -> crate::error::SdkResult<Merchant, GetMerchantErrorBody> {
        let path = format!("/v1/merchants/{}", merchant_code.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        if let Some(ref value) = params.version {
            request = request.query(&[("version", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: Merchant = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::api(GetMerchantErrorBody::NotFound))
            }
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// List Persons
    ///
    /// Returns a list of persons related to the merchant.
    pub async fn list_persons(
        &self,
        merchant_code: impl Into<String>,
        params: ListPersonsParams,
    ) -> crate::error::SdkResult<ListPersonsResponseBody, ListPersonsErrorBody> {
        let path = format!("/v1/merchants/{}/persons", merchant_code.into());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        if let Some(ref value) = params.version {
            request = request.query(&[("version", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListPersonsResponseBody = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::api(ListPersonsErrorBody::NotFound))
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err(crate::error::SdkError::api(
                ListPersonsErrorBody::InternalServerError,
            )),
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
    /// Retrieve a Person
    ///
    /// Returns a single person related to the merchant.
    pub async fn get_person(
        &self,
        merchant_code: impl Into<String>,
        person_id: impl Into<String>,
        params: GetPersonParams,
    ) -> crate::error::SdkResult<Person, GetPersonErrorBody> {
        let path = format!(
            "/v1/merchants/{}/persons/{}",
            merchant_code.into(),
            person_id.into()
        );
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(token) = self.client.authorization_token() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        if let Some(ref value) = params.version {
            request = request.query(&[("version", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: Person = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::api(GetPersonErrorBody::NotFound))
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err(crate::error::SdkError::api(
                GetPersonErrorBody::InternalServerError,
            )),
            _ => {
                let body_bytes = response.bytes().await?;
                let body = crate::error::UnknownApiBody::from_bytes(body_bytes.as_ref());
                Err(crate::error::SdkError::unexpected(status, body))
            }
        }
    }
}
