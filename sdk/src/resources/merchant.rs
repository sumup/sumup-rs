// The contents of this file are generated; do not modify them.

use super::common::*;
/// Profile information.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountLegacy {
    /// Username of the user profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The role of the user.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
/// Details of the registered address.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AddressWithDetails {
    /// Address line 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// Address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// City
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country ISO 3166-1 code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Country region id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<f64>,
    /// Region name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// Region code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    /// Postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    /// Landline number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landline: Option<String>,
    /// undefined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// undefined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// undefined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_details: Option<CountryDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeoffset_details: Option<TimeoffsetDetails>,
    /// undefined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_id: Option<String>,
}
/// Mobile app settings
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    /// Checkout preference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_preference: Option<String>,
    /// Include vat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_vat: Option<bool>,
    /// Manual entry tutorial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_entry_tutorial: Option<bool>,
    /// Mobile payment tutorial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_payment_tutorial: Option<bool>,
    /// Tax enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_enabled: Option<bool>,
    /// Mobile payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_payment: Option<String>,
    /// Reader payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_payment: Option<String>,
    /// Cash payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_payment: Option<String>,
    /// Advanced mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_mode: Option<String>,
    /// Expected max transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_max_transaction_amount: Option<f64>,
    /// Manual entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_entry: Option<String>,
    /// Terminal mode tutorial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_mode_tutorial: Option<bool>,
    /// Tipping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<String>,
    /// Tip rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_rates: Option<Vec<f64>>,
    /// Barcode scanner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode_scanner: Option<String>,
    /// Referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BankAccount {
    /// Bank code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Branch code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// SWIFT code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<String>,
    /// Account number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// IBAN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// Type of the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Account category - business or personal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    /// Status in the verification process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The primary bank account is the one used for payouts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    /// Creation date of the bank account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Bank name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
}
pub type BusinessOwners = Vec<serde_json::Value>;
/// Country Details
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CountryDetails {
    /// Currency ISO 4217 code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Country ISO code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_code: Option<String>,
    /// Country EN name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// Country native name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}
/// Doing Business As information
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DoingBusinessAsLegacy {
    /// Doing business as name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Doing business as company registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_number: Option<String>,
    /// Doing business as VAT ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    /// Doing business as website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// Doing business as email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<DoingBusinessAsLegacyAddress>,
}
/// Id of the legal type of the merchant profile
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LegalTypeLegacy {
    /// Unique id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    /// Legal type description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    /// Legal type short description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Sole trader legal type if true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sole_trader: Option<bool>,
}
/// Details of the merchant account.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MerchantAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountLegacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_profile: Option<PersonalProfileLegacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_profile: Option<MerchantProfileLegacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_settings: Option<AppSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsLegacy>,
}
/// Account's merchant profile
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MerchantProfileLegacy {
    /// Unique identifying code of the merchant profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    /// Company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_type: Option<LegalTypeLegacy>,
    /// Merchant category code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    /// Mobile phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    /// Company registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_number: Option<String>,
    /// Vat ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    /// Permanent certificate access code &#40;Portugal&#41;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_certificate_access_code: Option<String>,
    /// Nature and purpose of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature_and_purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressWithDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_owners: Option<BusinessOwners>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doing_business_as: Option<DoingBusinessAsLegacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MerchantSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rates: Option<VatRates>,
    /// Merchant locale &#40;for internal usage only&#41;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_accounts: Option<Vec<BankAccount>>,
    /// True if the merchant is extdev
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extdev: Option<bool>,
    /// True if the payout zone of this merchant is migrated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_zone_migrated: Option<bool>,
    /// Merchant country code formatted according to [ISO3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) &#40;for internal usage only&#41;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
/// Merchant settings &#40;like \"payout_type\", \"payout_period\"&#41;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MerchantSettings {
    /// Whether to show tax in receipts &#40;saved per transaction&#41;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_enabled: Option<bool>,
    /// Payout type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_type: Option<String>,
    /// Payout frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_period: Option<String>,
    /// Whether merchant can edit payouts on demand
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_on_demand_available: Option<bool>,
    /// Whether merchant will receive payouts on demand
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_on_demand: Option<bool>,
    /// Whether to show printers in mobile app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printers_enabled: Option<bool>,
    /// Payout Instrument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_instrument: Option<String>,
    /// Whether merchant can make MOTO payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto_payment: Option<String>,
    /// Stone merchant code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stone_merchant_code: Option<String>,
    /// Whether merchant will receive daily payout emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_payout_email: Option<bool>,
    /// Whether merchant will receive monthly payout emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_payout_email: Option<bool>,
    /// Whether merchant has gross settlement enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_settlement: Option<bool>,
}
/// User permissions
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PermissionsLegacy {
    /// Create MOTO payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_moto_payments: Option<bool>,
    /// Can view full merchant transaction history
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_transaction_history_view: Option<bool>,
    /// Refund transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_transactions: Option<bool>,
    /// Create referral
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_referral: Option<bool>,
}
/// Account's personal profile.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PersonalProfileLegacy {
    /// First name of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Mobile phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressWithDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}
/// TimeOffset Details
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TimeoffsetDetails {
    /// Postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    /// UTC offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    /// Daylight Saving Time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst: Option<bool>,
}
/// Merchant VAT rates
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct VatRates {
    /// Internal ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    /// Ordering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<f64>,
    /// Country ISO code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DoingBusinessAsLegacyAddress {
    /// Address line 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// Address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// City
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country ISO 3166-1 code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Country region ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<f64>,
    /// Country region name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// Postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetAccountParams {
    /// A list of additional information you want to receive for the user. By default only personal and merchant profile information will be returned.
    #[serde(rename = "include[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListBankAccountsParams {
    /// If true only the primary bank account (the one used for payouts) will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
/// OK
pub type ListBankAccountsResponse = Vec<BankAccount>;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ListBankAccountsV11Params {
    /// If true only the primary bank account (the one used for payouts) will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
/// OK
pub type ListBankAccountsV11Response = Vec<BankAccount>;
use crate::client::Client;
#[derive(Debug)]
pub enum GetMerchantProfileErrorBody {
    Status401(Error),
    Status403(ErrorForbidden),
}
#[derive(Debug)]
pub enum ListBankAccountsErrorBody {
    Status401(Error),
    Status403(ErrorForbidden),
}
#[derive(Debug)]
pub enum GetSettingsErrorBody {
    Status401(Error),
    Status403(ErrorForbidden),
}
#[derive(Debug)]
pub enum ListBankAccountsV11ErrorBody {
    Status401(Error),
    Status403(ErrorForbidden),
}
///Client for the Merchant API endpoints.
#[derive(Debug)]
pub struct MerchantClient<'a> {
    client: &'a Client,
}
impl<'a> MerchantClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// Retrieve a profile
    ///
    /// Returns user profile information.
    pub async fn get(
        &self,
        params: GetAccountParams,
    ) -> crate::error::SdkResult<MerchantAccount, Error> {
        let path = "/v0.1/me";
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
        if let Some(ref value) = params.include {
            request = request.query(&[("include[]", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: MerchantAccount = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::UNAUTHORIZED,
                    body,
                ))
            }
            _ => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api_raw(status, body))
            }
        }
    }
    /// Retrieve a merchant profile
    ///
    /// Retrieves merchant profile data.
    pub async fn get_merchant_profile(
        &self,
    ) -> crate::error::SdkResult<MerchantProfileLegacy, GetMerchantProfileErrorBody> {
        let path = "/v0.1/me/merchant-profile";
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
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: MerchantProfileLegacy = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::UNAUTHORIZED,
                    GetMerchantProfileErrorBody::Status401(body),
                ))
            }
            reqwest::StatusCode::FORBIDDEN => {
                let body: ErrorForbidden = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::FORBIDDEN,
                    GetMerchantProfileErrorBody::Status403(body),
                ))
            }
            _ => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api_raw(status, body))
            }
        }
    }
    /// List bank accounts
    ///
    /// Retrieves bank accounts of the merchant.
    pub async fn list_bank_accounts_deprecated(
        &self,
        params: ListBankAccountsParams,
    ) -> crate::error::SdkResult<ListBankAccountsResponse, ListBankAccountsErrorBody> {
        let path = "/v0.1/me/merchant-profile/bank-accounts";
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
        if let Some(ref value) = params.primary {
            request = request.query(&[("primary", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListBankAccountsResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::UNAUTHORIZED,
                    ListBankAccountsErrorBody::Status401(body),
                ))
            }
            reqwest::StatusCode::FORBIDDEN => {
                let body: ErrorForbidden = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::FORBIDDEN,
                    ListBankAccountsErrorBody::Status403(body),
                ))
            }
            _ => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api_raw(status, body))
            }
        }
    }
    /// Retrieve DBA
    ///
    /// Retrieves Doing Business As profile.
    pub async fn get_doing_business_as(
        &self,
    ) -> crate::error::SdkResult<DoingBusinessAsLegacy, Error> {
        let path = "/v0.1/me/merchant-profile/doing-business-as";
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
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: DoingBusinessAsLegacy = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::UNAUTHORIZED,
                    body,
                ))
            }
            _ => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api_raw(status, body))
            }
        }
    }
    /// Get settings
    ///
    /// Retrieves merchant settings.
    pub async fn get_settings(
        &self,
    ) -> crate::error::SdkResult<MerchantSettings, GetSettingsErrorBody> {
        let path = "/v0.1/me/merchant-profile/settings";
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
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: MerchantSettings = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::UNAUTHORIZED,
                    GetSettingsErrorBody::Status401(body),
                ))
            }
            reqwest::StatusCode::FORBIDDEN => {
                let body: ErrorForbidden = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::FORBIDDEN,
                    GetSettingsErrorBody::Status403(body),
                ))
            }
            _ => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api_raw(status, body))
            }
        }
    }
    /// Retrieve a personal profile
    ///
    /// Retrieves personal profile data.
    pub async fn get_personal_profile(
        &self,
    ) -> crate::error::SdkResult<PersonalProfileLegacy, Error> {
        let path = "/v0.1/me/personal-profile";
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
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: PersonalProfileLegacy = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::UNAUTHORIZED,
                    body,
                ))
            }
            _ => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api_raw(status, body))
            }
        }
    }
    /// List bank accounts
    ///
    /// Retrieves bank accounts of the merchant.
    pub async fn list_bank_accounts(
        &self,
        merchant_code: impl Into<String>,
        params: ListBankAccountsV11Params,
    ) -> crate::error::SdkResult<ListBankAccountsV11Response, ListBankAccountsV11ErrorBody> {
        let path = format!("/v1.1/merchants/{}/bank-accounts", merchant_code.into());
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
        if let Some(ref value) = params.primary {
            request = request.query(&[("primary", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: ListBankAccountsV11Response = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let body: Error = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::UNAUTHORIZED,
                    ListBankAccountsV11ErrorBody::Status401(body),
                ))
            }
            reqwest::StatusCode::FORBIDDEN => {
                let body: ErrorForbidden = response.json().await?;
                Err(crate::error::SdkError::api_parsed(
                    reqwest::StatusCode::FORBIDDEN,
                    ListBankAccountsV11ErrorBody::Status403(body),
                ))
            }
            _ => {
                let body = response.text().await?;
                Err(crate::error::SdkError::api_raw(status, body))
            }
        }
    }
}
