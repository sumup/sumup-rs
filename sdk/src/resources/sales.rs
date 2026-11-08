// The contents of this file are generated; do not modify them.

//! Operations related to reading sales data.
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum SaleCompleted {}
impl crate::event::private::Sealed for SaleCompleted {}
impl crate::events::EventSpec for SaleCompleted {
    const EVENT_TYPE: &'static str = "sales.completed";
    type FetchedObject = Sale;
}
/// Event notification for this event type.
pub type SaleCompletedEvent = crate::events::Event<SaleCompleted>;
/// Marker type for this event notification.
#[derive(Debug, Clone)]
pub enum SaleUpdated {}
impl crate::event::private::Sealed for SaleUpdated {}
impl crate::events::EventSpec for SaleUpdated {
    const EVENT_TYPE: &'static str = "sales.updated";
    type FetchedObject = Sale;
}
/// Event notification for this event type.
pub type SaleUpdatedEvent = crate::events::Event<SaleUpdated>;
/// A category a line belongs to.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct CategoryV10 {
    /// The category id. Usually a UUID, but not always (legacy cases).
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `b064ed44-66e1-45a1-a963-b783808f0436`
    #[builder(setter(into, doc = "Sets [`id`](CategoryV10::id)."))]
    pub id: String,
    /// The category name.
    ///
    /// Constraints:
    /// - max length: 30
    ///
    /// Example: `Food`
    #[builder(setter(into, doc = "Sets [`name`](CategoryV10::name)."))]
    pub name: String,
}
/// Method of applying the discount to the items or cart.
///
/// Example: `MANUAL`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DiscountApplicationMode {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "AUTOMATIC")]
    Automatic,
    #[serde(untagged)]
    Other(String),
}
/// The method used to target items to discount.
/// Must be populated for ITEM level discounts, and must be null for CART level discounts (CART level targets all
/// items, therefore none of these categories apply).
///
/// Example: `ITEM`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DiscountClass {
    #[serde(rename = "ITEM")]
    Item,
    #[serde(rename = "CATEGORY")]
    Category,
    #[serde(rename = "BUNDLE")]
    Bundle,
    #[serde(rename = "BUY_X_GET_Y")]
    BuyXGetY,
    #[serde(rename = "MEAL_DEAL")]
    MealDeal,
    #[serde(untagged)]
    Other(String),
}
/// Object representing information about single discount line.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct DiscountLine {
    /// A description or reason of the discount.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `Happy Hour`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`description`](DiscountLine::description)."
        )
    )]
    pub description: Option<String>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`gross_amount`](DiscountLine::gross_amount)."))]
    pub gross_amount: i64,
    /// An id used to associate a discount line with a specific discount.
    /// Since we can apply multiple discounts (e.g. 2x 10% whole cart off)
    /// we need to be able to identify which discount line belongs to which cart discount.
    /// Recommended to use UUID identifiers.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `075b0b5c-a1f4-46da-9aca-92fb6dd5905d`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`id`](DiscountLine::id).")
    )]
    pub id: Option<String>,
    /// Specifies where discount was applied by user (either merchant or consumer):
    /// - CART: Applied to the whole cart (calculated and distributed per sale item)
    /// - ITEM: Applied to specific sale item
    ///
    /// *Accepted values:* ["ITEM", "CART"].
    ///
    /// Constraints:
    /// - min length: 4
    /// - max length: 4
    ///
    /// Example: `CART`
    #[builder(setter(into, doc = "Sets [`level`](DiscountLine::level)."))]
    pub level: String,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`net_amount`](DiscountLine::net_amount)."))]
    pub net_amount: i64,
    #[builder(setter(doc = "Sets [`source`](DiscountLine::source)."))]
    pub source: DiscountSource,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`tax_amount`](DiscountLine::tax_amount)."))]
    pub tax_amount: i64,
    #[builder(setter(doc = "Sets [`value`](DiscountLine::value)."))]
    pub value: DiscountValue,
}
/// The source of discount. Exactly one property must be set.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct DiscountSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`ad_hoc`](DiscountSource::ad_hoc).")
    )]
    pub ad_hoc: Option<DiscountSourceAdHoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`basic`](DiscountSource::basic).")
    )]
    pub basic: Option<DiscountSourceBasic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`coupon`](DiscountSource::coupon).")
    )]
    pub coupon: Option<DiscountSourceCoupon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`loyalty`](DiscountSource::loyalty).")
    )]
    pub loyalty: Option<DiscountSourceLoyalty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`offer`](DiscountSource::offer).")
    )]
    pub offer: Option<DiscountSourceOffer>,
}
/// Discount created ad-hoc on device during checkout.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct DiscountSourceAdHoc {
    /// The description of the ad-hoc discount.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `Happy Hour from 10 to 12.`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`description`](DiscountSourceAdHoc::description)."
        )
    )]
    pub description: Option<String>,
    /// The unique identifier of the ad-hoc discount.
    ///
    /// Constraints:
    /// - format: `uuid`
    ///
    /// Example: `075b0b5c-a1f4-46da-9aca-92fb6dd5905d`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`id`](DiscountSourceAdHoc::id).")
    )]
    pub id: Option<String>,
    /// The name of the ad-hoc discount.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `Happy Hour`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`name`](DiscountSourceAdHoc::name).")
    )]
    pub name: Option<String>,
}
/// Preset discount applied manually on the POS.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct DiscountSourceBasic {
    #[builder(setter(doc = "Sets [`application_mode`](DiscountSourceBasic::application_mode)."))]
    pub application_mode: DiscountApplicationMode,
    /// The description of the loyalty discount.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `10% off food for staff members.`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`description`](DiscountSourceBasic::description)."
        )
    )]
    pub description: Option<crate::Nullable<String>>,
    /// The unique identifier of the basic discount.
    ///
    /// Constraints:
    /// - format: `uuid`
    ///
    /// Example: `075b0b5c-a1f4-46da-9aca-92fb6dd5905d`
    #[builder(setter(into, doc = "Sets [`id`](DiscountSourceBasic::id)."))]
    pub id: String,
    /// The name of the basic discount.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `Staff discount`
    #[builder(setter(into, doc = "Sets [`name`](DiscountSourceBasic::name)."))]
    pub name: String,
}
/// Discount applied manually when entering a coupon code at checkout.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct DiscountSourceCoupon {
    #[builder(setter(doc = "Sets [`application_mode`](DiscountSourceCoupon::application_mode)."))]
    pub application_mode: DiscountApplicationMode,
    /// The coupon code.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `FREE_COFFEE`
    #[builder(setter(into, doc = "Sets [`code`](DiscountSourceCoupon::code)."))]
    pub code: String,
    /// The description of the coupon discount.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `Free coffee with this coupon.`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`description`](DiscountSourceCoupon::description)."
        )
    )]
    pub description: Option<crate::Nullable<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`discount_class`](DiscountSourceCoupon::discount_class)."
        )
    )]
    pub discount_class: Option<DiscountClass>,
    /// The unique identifier of the coupon discount.
    ///
    /// Constraints:
    /// - format: `uuid`
    ///
    /// Example: `075b0b5c-a1f4-46da-9aca-92fb6dd5905d`
    #[builder(setter(into, doc = "Sets [`id`](DiscountSourceCoupon::id)."))]
    pub id: String,
    /// The name of the coupon discount.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `Free coffee`
    #[builder(setter(into, doc = "Sets [`name`](DiscountSourceCoupon::name)."))]
    pub name: String,
}
/// Discount applied when redeeming a reward from the merchant's loyalty scheme.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct DiscountSourceLoyalty {
    #[builder(setter(
        doc = "Sets [`application_mode`](DiscountSourceLoyalty::application_mode)."
    ))]
    pub application_mode: DiscountApplicationMode,
    /// The description of the loyalty discount.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `Free coffee for 50 points`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`description`](DiscountSourceLoyalty::description)."
        )
    )]
    pub description: Option<crate::Nullable<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`discount_class`](DiscountSourceLoyalty::discount_class)."
        )
    )]
    pub discount_class: Option<DiscountClass>,
    /// The unique identifier of the promotion or reward.
    ///
    /// Constraints:
    /// - format: `uuid`
    ///
    /// Example: `075b0b5c-a1f4-46da-9aca-92fb6dd5905d`
    #[builder(setter(into, doc = "Sets [`id`](DiscountSourceLoyalty::id)."))]
    pub id: String,
    #[builder(setter(doc = "Sets [`loyalty_type`](DiscountSourceLoyalty::loyalty_type)."))]
    pub loyalty_type: DiscountSourceLoyaltyType,
    /// The name of the loyalty discount.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `Free coffee`
    #[builder(setter(into, doc = "Sets [`name`](DiscountSourceLoyalty::name)."))]
    pub name: String,
    /// The number of points used for this loyalty discount.
    /// Must be a positive integer if the scheme type is POINTS.
    /// Must not be populated if the scheme type is STAMPS.
    ///
    /// Constraints:
    /// - value >= 1
    ///
    /// Example: `50`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`points`](DiscountSourceLoyalty::points).")
    )]
    pub points: Option<crate::Nullable<i64>>,
    #[builder(setter(doc = "Sets [`scheme_type`](DiscountSourceLoyalty::scheme_type)."))]
    pub scheme_type: DiscountSourceLoyaltySchemeType,
}
/// The scheme type of the loyalty program.
///
/// Example: `POINTS`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DiscountSourceLoyaltySchemeType {
    #[serde(rename = "STAMPS")]
    Stamps,
    #[serde(rename = "POINTS")]
    Points,
    #[serde(untagged)]
    Other(String),
}
/// The type of loyalty discount, either a promotion (e.g. new customer discount) or a reward (free coffee after completing stamp card).
///
/// Example: `REWARD`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DiscountSourceLoyaltyType {
    #[serde(rename = "PROMOTION")]
    Promotion,
    #[serde(rename = "REWARD")]
    Reward,
    #[serde(untagged)]
    Other(String),
}
/// Discount applied automatically when cart is updated based on preset promotions.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct DiscountSourceOffer {
    #[builder(setter(doc = "Sets [`application_mode`](DiscountSourceOffer::application_mode)."))]
    pub application_mode: DiscountApplicationMode,
    /// The description of the offer discount.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `Buy a Croissant, get a coffee for £1`
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::nullable::deserialize"
    )]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`description`](DiscountSourceOffer::description)."
        )
    )]
    pub description: Option<crate::Nullable<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`discount_class`](DiscountSourceOffer::discount_class)."
        )
    )]
    pub discount_class: Option<DiscountClass>,
    /// The unique identifier of the offer discount.
    ///
    /// Constraints:
    /// - format: `uuid`
    ///
    /// Example: `075b0b5c-a1f4-46da-9aca-92fb6dd5905d`
    #[builder(setter(into, doc = "Sets [`id`](DiscountSourceOffer::id)."))]
    pub id: String,
    /// The name of the offer discount.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `Breakfast combo deal`
    #[builder(setter(into, doc = "Sets [`name`](DiscountSourceOffer::name)."))]
    pub name: String,
}
/// The value type of the discount, either a percentage based or absolute monetary value.
///
/// It is required that only one of the following is set:
/// - absolute
/// - percentage.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct DiscountValue {
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`absolute`](DiscountValue::absolute).")
    )]
    pub absolute: Option<i64>,
    /// The percentage in minor units (1/100000).
    ///
    /// Examples:
    /// - 10% -> 0.10 (decimal) = 10000 (minor units)
    /// - 15.5% -> 0.155 (decimal) = 15500 (minor units)
    /// - 0.001% -> 0.00001 (decimal) = 1 (minor units)
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `10000`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`percentage`](DiscountValue::percentage).")
    )]
    pub percentage: Option<i64>,
}
/// A named, typed characteristic of a line. New item types add data here instead of
/// a typed sub-object per type.
///
/// Examples by item type (illustrative, not exhaustive):
/// - DEPOSIT:   `operation_type` = "SALE", `linked_line_id` = "line-1"
/// - GIFT_CARD: `gift_card_reference` = "GCMINGXPC5"
/// - CATALOG:   `variant_name` = "Large", `item_id` / `variant_id` (TEXT),
/// `reporting_category_id` = "cat-..."
/// - FEE:       `fee_type` = "SHIPPING" (required); optional `fee_id`,
/// `value_absolute` or `value_percentage`, and per-type metadata
/// features such as `delivery_name` / `tracking_reference` (shipping),
/// `card_type` / `scheme` (surcharge), or `policy_id` (cancellation)
/// - (future) SUBSCRIPTION: `billing_period` = "P1M", `renews_at` = "2026-07-02"
///
/// Conventional `FEE` feature keys:
/// - `fee_type` (TEXT, required): `SHIPPING`, `SURCHARGE`, `SERVICE_CHARGE`, or
/// `CANCELLATION`
/// - `fee_id` (TEXT, optional): identifier of the fee definition
/// - `value_absolute` (INTEGER, optional): configured absolute value in minor units
/// of the sale currency (the charged amount is on `price`)
/// - `value_percentage` (INTEGER, optional): configured percentage in minor units
/// (1/100000; e.g. 10000 = 10%)
/// - Per-type metadata (optional, TEXT unless noted): `delivery_name`,
/// `tracking_reference` (shipping); `card_type`, `scheme` (surcharge);
/// `policy_id` (cancellation)
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct FeatureV10 {
    /// The feature name/key.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `operation_type`
    #[builder(setter(into, doc = "Sets [`name`](FeatureV10::name)."))]
    pub name: String,
    /// The feature value in canonical string form, to be parsed per `value_type`
    /// (e.g. INTEGER -> "3", DECIMAL -> "1.50", DATE -> "2026-07-02",
    /// BOOLEAN -> "true"). Carrying it as a string keeps the wire format stable as
    /// new feature names and value types are introduced.
    ///
    /// Constraints:
    /// - max length: 1000
    ///
    /// Example: `SALE`
    #[builder(setter(into, doc = "Sets [`value`](FeatureV10::value)."))]
    pub value: String,
    #[builder(setter(doc = "Sets [`value_type`](FeatureV10::value_type)."))]
    pub value_type: FeatureValueTypeV10,
}
/// The data type of a feature value. The wire `value` is always a string and is parsed
/// according to this type.
///
/// Example: `TEXT`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FeatureValueTypeV10 {
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "DECIMAL")]
    Decimal,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(untagged)]
    Other(String),
}
/// Final price amounts after all calculations and adjustments.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct FinalPriceAmounts {
    /// Discounts applied.
    ///
    /// Important:
    /// Discount amounts shouldn't take effect on net and gross amounts.
    ///
    /// Discount money amounts depend on where discount was applied:
    /// - for item level discount: whole discount money amount is applied
    /// - for cart level discount: only part of the whole amount is applied
    /// (discount money value is split on each line item)
    ///
    /// Constraints:
    /// - max items: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`discounts`](FinalPriceAmounts::discounts)."
        )
    )]
    pub discounts: Option<Vec<DiscountLine>>,
    /// Taxes applied on top of `total_net_amount_final` amount resulting in `total_gross_amount_final` amount.
    /// If no discounts are applied, it should be equal to `taxes`.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 1000
    #[builder(setter(doc = "Sets [`taxes_final`](FinalPriceAmounts::taxes_final)."))]
    pub taxes_final: Vec<PositiveTaxLine>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(
        doc = "Sets [`total_gross_amount_final`](FinalPriceAmounts::total_gross_amount_final)."
    ))]
    pub total_gross_amount_final: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(
        doc = "Sets [`total_net_amount_final`](FinalPriceAmounts::total_net_amount_final)."
    ))]
    pub total_net_amount_final: i64,
}
/// Outward identifiers and links for a line, kept in one place rather than scattered
/// per item type.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct ItemReferencesV10 {
    /// Reference to the backing catalog entity (item variant, deposit, ...),
    /// typically a URN such as
    /// `urn:sumup:pos:variant:${MERCHANT_CODE}:${ITEM_ID}:${VARIANT_ID}` or
    /// `urn:sumup:pos:deposit:${MERCHANT_CODE}:${DEPOSIT_ID}`.
    /// Empty for ad-hoc / manual lines. Type-agnostic on purpose: a new item type
    /// backed by a catalog entity reuses this field instead of adding its own.
    ///
    /// Constraints:
    /// - max length: 255
    ///
    /// Example: `urn:sumup:pos:variant:MCGLE7VC:9736e947-6f1f-487f-846a-c5954fb06a5a:7fff192f-ce29-4673-890f-2d68ba557946`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`catalog_reference`](ItemReferencesV10::catalog_reference)."
        )
    )]
    pub catalog_reference: Option<String>,
    /// The source of the catalog the line originates from.
    ///
    /// Example: `SUMUP`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`catalog_source`](ItemReferencesV10::catalog_source)."
        )
    )]
    pub catalog_source: Option<ItemReferencesV10CatalogSource>,
    /// An external reference id connecting this line to another system.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `POS-LINE-001`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`external_id`](ItemReferencesV10::external_id)."
        )
    )]
    pub external_id: Option<String>,
    /// Stock keeping unit, enriched from the Catalog when available.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `COFFEE-LARGE`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`sku`](ItemReferencesV10::sku).")
    )]
    pub sku: Option<String>,
}
/// Pricing for a single line, valid for every item type.
///
/// Merges the standard amounts (with tax breakdown) and the final amounts (after
/// discounts/modifiers).
///
/// Totals and tax amounts can be negative for return-like operations (for example
/// a deposit with `operation_type` = `RETURN`).
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct LineItemPriceV10 {
    /// Discounts applied. Discount amounts do not affect `total_net_amount` /
    /// `total_gross_amount`; they are reflected in the `*_final` amounts.
    ///
    /// Constraints:
    /// - max items: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`discounts`](LineItemPriceV10::discounts).")
    )]
    pub discounts: Option<Vec<DiscountLine>>,
    /// Taxes applied on top of the net amount resulting in the gross amount.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 1000
    #[builder(setter(doc = "Sets [`taxes`](LineItemPriceV10::taxes)."))]
    pub taxes: Vec<TaxLine>,
    /// Taxes applied on top of `total_net_amount_final` resulting in `total_gross_amount_final`. Equals `taxes` if no discounts apply.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 1000
    #[builder(setter(doc = "Sets [`taxes_final`](LineItemPriceV10::taxes_final)."))]
    pub taxes_final: Vec<TaxLine>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`total_gross_amount`](LineItemPriceV10::total_gross_amount)."))]
    pub total_gross_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(
        doc = "Sets [`total_gross_amount_final`](LineItemPriceV10::total_gross_amount_final)."
    ))]
    pub total_gross_amount_final: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`total_net_amount`](LineItemPriceV10::total_net_amount)."))]
    pub total_net_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(
        doc = "Sets [`total_net_amount_final`](LineItemPriceV10::total_net_amount_final)."
    ))]
    pub total_net_amount_final: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`unit_amount`](LineItemPriceV10::unit_amount)."))]
    pub unit_amount: i64,
}
/// Modifiers are reported under line items and can be build from Item Catalog objects.
/// Modifiers has to be attached to an item and can't be sold and refunded alone.
pub type ModifierItem = serde_json::Value;
/// Modifier set attached to an item.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct ModifierSet {
    /// The id of the modifier set that modifier belongs to.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `b064ed44-66e1-45a1-a963-b783808f0436`
    #[builder(setter(into, doc = "Sets [`id`](ModifierSet::id)."))]
    pub id: String,
    /// The name of the modifier set that the modifier belongs to.
    ///
    /// Constraints:
    /// - min length: 1
    /// - max length: 150
    ///
    /// Example: `Burger Toppings`
    #[builder(setter(into, doc = "Sets [`name`](ModifierSet::name)."))]
    pub name: String,
}
/// Payment outcome filter value. Multi-select uses **OR** within this field; combined with
/// other filters using **AND**.
/// Only fully-paid closed sales are exposed. Refunded original sales remain searchable as
/// `PAID`; refunds are separate negative sales, not a `REFUNDED` sale-search outcome.
///
/// Example: `PAID`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PaymentStatusFilterV10 {
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "CHARGED_BACK")]
    ChargedBack,
    #[serde(untagged)]
    Other(String),
}
/// Price information for items with positive amounts only.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PositivePriceAmounts {
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(
        doc = "Sets [`total_gross_amount`](PositivePriceAmounts::total_gross_amount)."
    ))]
    pub total_gross_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`total_net_amount`](PositivePriceAmounts::total_net_amount)."))]
    pub total_net_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`unit_amount`](PositivePriceAmounts::unit_amount)."))]
    pub unit_amount: i64,
}
/// Price information with positive amounts and detailed tax breakdown.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PositivePriceWithTaxes {
    /// Taxes applied on top of net amount resulting in gross amount.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 1000
    #[builder(setter(doc = "Sets [`taxes`](PositivePriceWithTaxes::taxes)."))]
    pub taxes: Vec<PositiveTaxLine>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(
        doc = "Sets [`total_gross_amount`](PositivePriceWithTaxes::total_gross_amount)."
    ))]
    pub total_gross_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(
        doc = "Sets [`total_net_amount`](PositivePriceWithTaxes::total_net_amount)."
    ))]
    pub total_net_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`unit_amount`](PositivePriceWithTaxes::unit_amount)."))]
    pub unit_amount: i64,
}
/// Tax line with positive amounts only.
/// Information about tax together with amounts before and after its application such as:
/// - net amount: taxable net amount, scoped by this tax
/// - gross amount: amount after applying the tax
/// The numbers here can't be negative.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PositiveTaxLine {
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`gross_amount`](PositiveTaxLine::gross_amount)."))]
    pub gross_amount: i64,
    /// Tax ID from the Tax Service (External Id)
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `b064ed44-66e1-45a1-a963-b783808f0436`
    #[builder(setter(into, doc = "Sets [`id`](PositiveTaxLine::id)."))]
    pub id: String,
    /// Tax name, e.g. "MwSt."
    ///
    /// Constraints:
    /// - max length: 255
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`name`](PositiveTaxLine::name).")
    )]
    pub name: Option<String>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`net_amount`](PositiveTaxLine::net_amount)."))]
    pub net_amount: i64,
    /// The tax rate percentage in minor units (1/100000). For example 19000 = 19%.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `19000`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`percentage`](PositiveTaxLine::percentage)."
        )
    )]
    pub percentage: Option<i64>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`tax_amount`](PositiveTaxLine::tax_amount)."))]
    pub tax_amount: i64,
}
/// Base price information for items including unit amount and totals.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PriceAmounts {
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`total_gross_amount`](PriceAmounts::total_gross_amount)."))]
    pub total_gross_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`total_net_amount`](PriceAmounts::total_net_amount)."))]
    pub total_net_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`unit_amount`](PriceAmounts::unit_amount)."))]
    pub unit_amount: i64,
}
/// A RFC 9457 problem details object.
///
/// Additional properties specific to the problem type may be present.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct ProblemV10 {
    /// A human-readable explanation specific to this occurrence of the problem.
    ///
    /// Example: `The requested resource doesn't exist or does not belong to you.`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`detail`](ProblemV10::detail).")
    )]
    pub detail: Option<String>,
    /// A URI reference that identifies the specific occurrence of the problem.
    ///
    /// Constraints:
    /// - format: `uri`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`instance`](ProblemV10::instance).")
    )]
    pub instance: Option<String>,
    /// The HTTP status code generated by the origin server for this occurrence of the problem.
    ///
    /// Example: `404`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`status`](ProblemV10::status).")
    )]
    pub status: Option<i64>,
    /// A short, human-readable summary of the problem type.
    ///
    /// Example: `Requested resource couldn't be found.`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`title`](ProblemV10::title).")
    )]
    pub title: Option<String>,
    /// A URI reference that identifies the problem type.
    ///
    /// Constraints:
    /// - format: `uri`
    ///
    /// Example: `https://developer.sumup.com/problem/not-found`
    #[serde(rename = "type")]
    #[builder(setter(into, doc = "Sets [`r#type`](ProblemV10::r#type)."))]
    pub r#type: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    #[builder(
        default,
        setter(doc = "Sets [`additional_properties`](ProblemV10::additional_properties).")
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
impl std::fmt::Display for ProblemV10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ProblemV10 {}
/// Card brand, credit/debit processing type, and last four digits.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PublicCardV10 {
    /// The brand of the card.
    /// Input is case-insensitive; the canonical form is lowercase snake_case.
    /// **VALUES**: [amex, cup, diners, discover, elo, elv, hipercard, jcb, maestro, mastercard, visa, visa_electron,
    /// visa_vpay, vr, eftpos, conecs, swile, girocard, interac, sumup, bancomat].
    ///
    /// Constraints:
    /// - max length: 20
    ///
    /// Example: `visa`
    #[builder(setter(into, doc = "Sets [`brand`](PublicCardV10::brand)."))]
    pub brand: String,
    /// The last 4 digits of the card used for the payment.
    ///
    /// Constraints:
    /// - pattern: `^\d{4}$`
    /// - min length: 4
    /// - max length: 4
    ///
    /// Example: `1234`
    #[builder(setter(into, doc = "Sets [`last_4_digits`](PublicCardV10::last_4_digits)."))]
    pub last_4_digits: String,
    /// Credit or debit processing choice, not brand or funding.
    /// `unknown` when unavailable; prepaid/voucher are not processing types.
    ///
    /// Example: `debit`
    #[serde(rename = "type")]
    #[builder(setter(doc = "Sets [`r#type`](PublicCardV10::r#type)."))]
    pub r#type: PublicCardV10Type,
}
/// Gift card used for this payment. Present when `method` is `gift_card` and the gift card
/// id is known.
/// Gift card IDs use the format "GC" followed by 9 alphanumeric characters
/// (e.g. GCZ3NTMB1V).
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PublicGiftCardPayment {
    /// Gift card identifier. Format "GC" followed by 9 alphanumeric characters
    /// (e.g. GCZ3NTMB1V).
    ///
    /// Example: `GCZ3NTMB1V`
    #[builder(setter(into, doc = "Sets [`id`](PublicGiftCardPayment::id)."))]
    pub id: String,
}
/// Payment on a closed sale, for bookkeeping and reconciliation.
///
/// Acquirer/processing cost is not on this object. Merchant-charged fees (shipping, service
/// charge, etc.) appear as `type: "FEE"` item lines in `Sale.items`.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct PublicPaymentV10 {
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`amount`](PublicPaymentV10::amount)."))]
    pub amount: i64,
    /// Alternative payment method type when `method` is `apm`.
    /// Gift card is not an APM on this API.
    /// Input is case-insensitive; the canonical form is lowercase snake_case.
    /// **VALUES**: [qr_code_pix, sepa_dd, paypal, sumup_pay, twint, iris].
    ///
    /// Constraints:
    /// - max length: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`apm_type`](PublicPaymentV10::apm_type)."
        )
    )]
    pub apm_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`card`](PublicPaymentV10::card).")
    )]
    pub card: Option<PublicCardV10>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`gift_card`](PublicPaymentV10::gift_card).")
    )]
    pub gift_card: Option<PublicGiftCardPayment>,
    /// Payment method on public sales read API v1.0.
    /// Digital wallets are first-class methods. Gift card payments use `gift_card`, not `apm`.
    /// Input is case-insensitive; the canonical form is lowercase snake_case.
    /// **VALUES**: [card, cash, apm, external, apple_pay, google_pay, whatsapp_pay, gift_card].
    ///
    /// Constraints:
    /// - max length: 20
    ///
    /// Example: `card`
    #[builder(setter(into, doc = "Sets [`method`](PublicPaymentV10::method)."))]
    pub method: String,
    /// UTC date-time when SumUp recorded this payment’s exposed `status` (successful, failed,
    /// or canceled). Server processed time when available; otherwise the payment provider’s
    /// transaction time. Not the device clock.
    ///
    /// Example: `2017-07-21T17:32:28+01:00`
    #[builder(setter(doc = "Sets [`processed_at`](PublicPaymentV10::processed_at)."))]
    pub processed_at: crate::datetime::DateTime,
    /// The reference to the actual payment used for this sale, e.g. the sumup transaction code.
    ///
    /// Constraints:
    /// - max length: 50
    ///
    /// Example: `TERTYFQ4G3`
    #[builder(setter(into, doc = "Sets [`reference`](PublicPaymentV10::reference)."))]
    pub reference: String,
    /// The status of the payment.
    ///
    /// Example: `SUCCESSFUL`
    #[builder(setter(doc = "Sets [`status`](PublicPaymentV10::status)."))]
    pub status: PublicPaymentV10Status,
    /// Payment operation type.
    /// This is distinct from the payment instrument in `method`.
    /// Input is case-insensitive; the canonical form is lowercase snake_case.
    /// **VALUES**: [purchase, unreferenced_refund]
    /// Defaults to "purchase" if unspecified. Either a normal purchase or an unreferenced refund for negative
    /// sales like deposit returns.
    ///
    /// Example: `purchase`
    #[serde(rename = "type")]
    #[builder(setter(into, doc = "Sets [`r#type`](PublicPaymentV10::r#type)."))]
    pub r#type: String,
}
/// Core model of the Sales API. Returned by `GET /sales` and `GET /sales/{sale_id}`.
///
/// Non-finalised sales are never returned; only fully-paid sales.
///
/// Item lines are `SaleLineItem`. Merchant fees appear as `type: "FEE"` lines; extra fee data
/// is in `features`.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct Sale {
    /// Instant when the sale was closed. Always present: non-finalised sales are never returned.
    ///
    /// Example: `2026-06-02T10:20:00Z`
    #[builder(setter(doc = "Sets [`closed_at`](Sale::closed_at)."))]
    pub closed_at: crate::datetime::DateTime,
    /// Instant when the sale was created.
    ///
    /// Example: `2026-06-02T10:15:00Z`
    #[builder(setter(doc = "Sets [`created_at`](Sale::created_at)."))]
    pub created_at: crate::datetime::DateTime,
    #[builder(setter(doc = "Sets [`currency`](Sale::currency)."))]
    pub currency: SalesActivityCurrency,
    /// The ID of the sale.
    ///
    /// Example: `dbe3178d-4b83-5024-9b26-9b8e1b280514`
    #[builder(setter(into, doc = "Sets [`id`](Sale::id)."))]
    pub id: String,
    /// The lines sold in this sale, in the order they were added to the cart.
    /// Includes merchant fees (`type: "FEE"`) when present; fee details are in `features`.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 1000
    #[builder(setter(doc = "Sets [`items`](Sale::items)."))]
    pub items: Vec<SaleLineItemV10>,
    /// Human-readable name of the sale (not unique, not the sale id).
    ///
    /// Constraints:
    /// - max length: 160
    ///
    /// Example: `Order 1234`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`name`](Sale::name).")
    )]
    pub name: Option<String>,
    /// Optional note attached to the sale, visible on receipts and the UI.
    ///
    /// Constraints:
    /// - max length: 300
    ///
    /// Example: `Please deliver to the back door.`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`note`](Sale::note).")
    )]
    pub note: Option<String>,
    /// Payments on this sale, in the order they were recorded.
    ///
    /// Constraints:
    /// - max items: 1000
    #[builder(setter(doc = "Sets [`payments`](Sale::payments)."))]
    pub payments: Vec<PublicPaymentV10>,
    #[builder(setter(doc = "Sets [`price`](Sale::price)."))]
    pub price: SalePriceV10,
    #[builder(setter(doc = "Sets [`source`](Sale::source)."))]
    pub source: SaleSource,
    #[builder(setter(doc = "Sets [`state`](Sale::state)."))]
    pub state: SaleStateV10,
    #[builder(setter(doc = "Sets [`tax_mode`](Sale::tax_mode)."))]
    pub tax_mode: TaxMode,
    /// Identity UUID of the cashier who processed the sale. Omitted when the cashier
    /// identity is unknown.
    ///
    /// Constraints:
    /// - format: `uuid`
    ///
    /// Example: `d6f51684-519b-454c-bdb3-deeb05f52b92`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`user_id`](Sale::user_id).")
    )]
    pub user_id: Option<String>,
}
/// A single line of a sale.
///
/// The envelope (id, type, name, quantity, unit_of_measure, price) is stable and
/// type-agnostic: any consumer can render and total a line from these fields alone.
/// Type-specific data is in `features`. There is no per-type object and no discriminator,
/// so a new item type does not change this schema.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SaleLineItemV10 {
    /// Categories the line belongs to. A shared optional structure available to any
    /// line; not a per-type field.
    ///
    /// Constraints:
    /// - max items: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`categories`](SaleLineItemV10::categories)."
        )
    )]
    pub categories: Option<Vec<CategoryV10>>,
    /// Named, typed, type-specific data. New item types add their data here as
    /// features instead of new schema variants. See `Feature` for per-type examples.
    ///
    /// Constraints:
    /// - max items: 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`features`](SaleLineItemV10::features).")
    )]
    pub features: Option<Vec<FeatureV10>>,
    /// Stable identifier of the line within the sale. Used for linking (e.g. deposits).
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `line-1`
    #[builder(setter(into, doc = "Sets [`id`](SaleLineItemV10::id)."))]
    pub id: String,
    /// Modifiers customise a line (e.g. "oat milk" on a cappuccino). A shared
    /// optional structure available to any line; not a per-type field.
    ///
    /// Constraints:
    /// - max items: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`modifiers`](SaleLineItemV10::modifiers).")
    )]
    pub modifiers: Option<Vec<ModifierItem>>,
    /// Display name of the line.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `Cappuccino`
    #[builder(setter(into, doc = "Sets [`name`](SaleLineItemV10::name)."))]
    pub name: String,
    /// Optional free-text note attached to the line, visible on receipts.
    ///
    /// Constraints:
    /// - max length: 300
    ///
    /// Example: `extra hot`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`note`](SaleLineItemV10::note).")
    )]
    pub note: Option<String>,
    #[builder(setter(doc = "Sets [`price`](SaleLineItemV10::price)."))]
    pub price: LineItemPriceV10,
    /// The numerical value representing the quantity in the specified unit.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `2.15332`
    #[builder(setter(doc = "Sets [`quantity`](SaleLineItemV10::quantity)."))]
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`references`](SaleLineItemV10::references)."
        )
    )]
    pub references: Option<ItemReferencesV10>,
    /// The type of service for this line.
    ///
    /// Example: `IN_HOUSE`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`service_type`](SaleLineItemV10::service_type)."
        )
    )]
    pub service_type: Option<SaleLineItemV10ServiceType>,
    /// Classifies a sale line. Not a schema subclass.
    ///
    /// Known values: `CATALOG`, `DEPOSIT`, `GIFT_CARD`, `CUSTOM`, `FEE`.
    ///
    /// `FEE` lines represent merchant-charged fees on the sale (e.g. shipping, service
    /// charge, surcharge, cancellation). They participate in sale totals like any other
    /// line; type-specific data is carried in `features` (see `Feature` for the
    /// conventional feature keys).
    ///
    /// This is intentionally an open string, NOT a closed enum: new values are added
    /// over time as pure data and consumers MUST tolerate unknown values, rendering the
    /// line from the generic envelope (name, quantity, price, taxes) and reading any
    /// extra information from `features`.
    ///
    /// Example: `CATALOG`
    #[serde(rename = "type")]
    #[builder(setter(into, doc = "Sets [`r#type`](SaleLineItemV10::r#type)."))]
    pub r#type: String,
    #[builder(setter(doc = "Sets [`unit_of_measure`](SaleLineItemV10::unit_of_measure)."))]
    pub unit_of_measure: UnitOfMeasure,
}
/// Paginated list of `Sale` objects. Results are ordered **newest first** by the newest
/// activity recorded for each sale. See `ListSalesV10`.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SalePage {
    #[builder(setter(doc = "Sets [`items`](SalePage::items)."))]
    pub items: Vec<Sale>,
    /// Pagination token for subsequent requests. Should only be set by clients if they have
    /// received a token in the previous response, and this request is an immediate follow-up
    /// request to that response to load the subsequent page. Clients should treat this as an
    /// opaque token and should not attempt to parse it in any way.
    ///
    /// Example: `66bcbtzelrubzeuzruzgu76678trncH`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`pagination_token`](SalePage::pagination_token)."
        )
    )]
    pub pagination_token: Option<String>,
}
/// Sale-level pricing aggregate, parallel to `LineItemPrice` on each `SaleLineItem`.
///
/// Line-level breakdown (unit price, line discounts, per-line taxes) lives on
/// `items[].price`. This object holds the rolled-up totals and cart-level
/// adjustments that consumers use for receipts and payment reconciliation.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SalePriceV10 {
    /// Discounts applied at cart level. Item-level discounts remain on
    /// `items[].price.discounts`.
    ///
    /// Constraints:
    /// - max items: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`cart_discounts`](SalePriceV10::cart_discounts)."
        )
    )]
    pub cart_discounts: Option<Vec<DiscountLine>>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`gross_amount`](SalePriceV10::gross_amount)."))]
    pub gross_amount: i64,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`net_amount`](SalePriceV10::net_amount)."))]
    pub net_amount: i64,
    /// Tax lines aggregated across the sale (after cart-level discounts).
    /// Per-line tax detail remains on `items[].price.taxes` / `taxes_final`.
    ///
    /// Constraints:
    /// - min items: 1
    /// - max items: 1000
    #[builder(setter(doc = "Sets [`taxes`](SalePriceV10::taxes)."))]
    pub taxes: Vec<TaxLine>,
}
/// Summary row for a sale in search results. Use `GET /sales/{sale_id}` for the full `Sale`.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SaleSearchItem {
    /// Instant when the sale was closed. Always present: non-finalised sales are never returned.
    ///
    /// Example: `2026-06-01T14:35:00Z`
    #[builder(setter(doc = "Sets [`closed_at`](SaleSearchItem::closed_at)."))]
    pub closed_at: crate::datetime::DateTime,
    /// Instant when the sale was created.
    ///
    /// Example: `2026-06-01T14:30:00Z`
    #[builder(setter(doc = "Sets [`created_at`](SaleSearchItem::created_at)."))]
    pub created_at: crate::datetime::DateTime,
    #[builder(setter(doc = "Sets [`currency`](SaleSearchItem::currency)."))]
    pub currency: SalesActivityCurrency,
    /// Sale identifier.
    ///
    /// Example: `dbe3178d-4b83-5024-9b26-9b8e1b280514`
    #[builder(setter(into, doc = "Sets [`id`](SaleSearchItem::id)."))]
    pub id: String,
    /// Display name of the sale when set; otherwise a concatenation of item names with
    /// quantities. Omitted when neither is available.
    ///
    /// Example: `Order 1234`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`name`](SaleSearchItem::name).")
    )]
    pub name: Option<String>,
    #[builder(setter(doc = "Sets [`state`](SaleSearchItem::state)."))]
    pub state: SaleStateV10,
    #[builder(setter(doc = "Sets [`tax_mode`](SaleSearchItem::tax_mode)."))]
    pub tax_mode: TaxMode,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`total_amount`](SaleSearchItem::total_amount)."))]
    pub total_amount: i64,
    /// Identity UUID of the cashier who processed the sale. Omitted when the cashier
    /// identity is unknown.
    ///
    /// Constraints:
    /// - format: `uuid`
    ///
    /// Example: `d6f51684-519b-454c-bdb3-deeb05f52b92`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`user_id`](SaleSearchItem::user_id).")
    )]
    pub user_id: Option<String>,
}
/// Filter and pagination criteria for `POST /sales/search`. Filters from different categories
/// stack with **logical AND**; multiple values in one array field stack with **logical OR**.
/// Results cover the merchant’s **last two years** of sales (narrow further with `start_date` /
/// `end_date`). Results are sorted **newest first** by sale close date.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SaleSearchRequest {
    /// Sale created **on or before** this instant (inclusive).
    ///
    /// Example: `2026-06-02T23:59:59Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`end_date`](SaleSearchRequest::end_date).")
    )]
    pub end_date: Option<crate::datetime::DateTime>,
    /// Maximum number of results per page.
    ///
    /// Constraints:
    /// - value >= 1
    /// - value <= 25
    ///
    /// Example: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`pagination_limit`](SaleSearchRequest::pagination_limit)."
        )
    )]
    pub pagination_limit: Option<i64>,
    /// Opaque pagination token from a previous search response. Not interchangeable with tokens
    /// from `GET /sales`.
    ///
    /// Example: `66bcbtzelrubzeuzruzgu76678trncH`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`pagination_token`](SaleSearchRequest::pagination_token)."
        )
    )]
    pub pagination_token: Option<String>,
    /// Filters rows by payment outcome. Multiple values match any of them (**OR**). The search
    /// row does not echo this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`payment_statuses`](SaleSearchRequest::payment_statuses)."
        )
    )]
    pub payment_statuses: Option<Vec<PaymentStatusFilterV10>>,
    /// Filters rows by sale lifecycle (`state`). Multiple values match any of them (**OR**).
    /// Uses the same values as `Sale.state`. Non-finalised sales are never returned; only
    /// fully-paid sales.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`sale_statuses`](SaleSearchRequest::sale_statuses)."
        )
    )]
    pub sale_statuses: Option<Vec<SaleStateV10>>,
    /// Sales channel filter. Multiple values match any of them (**OR**).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`sales_channels`](SaleSearchRequest::sales_channels)."
        )
    )]
    pub sales_channels: Option<Vec<SalesChannelFilterV10>>,
    /// Sale created **on or after** this instant (inclusive).
    ///
    /// Example: `2026-06-01T00:00:00Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`start_date`](SaleSearchRequest::start_date)."
        )
    )]
    pub start_date: Option<crate::datetime::DateTime>,
}
/// Search hits and an optional pagination token. Results are ordered **newest first** by sale close date.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SaleSearchResponse {
    #[builder(setter(doc = "Sets [`items`](SaleSearchResponse::items)."))]
    pub items: Vec<SaleSearchItem>,
    /// Opaque token for the next page of search results.
    ///
    /// Example: `66bcbtzelrubzeuzruzgu76678trncH`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`pagination_token`](SaleSearchResponse::pagination_token)."
        )
    )]
    pub pagination_token: Option<String>,
}
/// Information about the source of the sale.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SaleSource {
    /// The sale_channel is field that indicates the channel through
    /// which the sale was made. For example, if the sale was made in the merchant app,
    /// the sale_channel could be POS. If the sale was made on a kiosk, the
    /// sale_channel is KIOSK.
    ///
    /// *Accepted values:* ["POS", "KIOSK", "ONLINE_STORE", "SOLO", "ORDER_AND_PAY", "INVOICE", "BOOKINGS", "ORDER_AGGREGATOR", "SUBSCRIPTIONS", "UNKNOWN"].
    ///
    /// Example: `POS`
    #[builder(setter(into, doc = "Sets [`channel`](SaleSource::channel)."))]
    pub channel: String,
    /// An external reference ID for connecting orders/sales between products
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `123456`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`external_id`](SaleSource::external_id)."
        )
    )]
    pub external_id: Option<String>,
    /// An external reference ID for the sub-channel.
    /// For example, when channel is BOOKINGS and sub_channel is POS,
    /// this can hold the OMS order ID while external_id holds the appointment ID.
    ///
    /// Constraints:
    /// - max length: 250
    ///
    /// Example: `9736e947-6f1f-487f-846a-c5954fb06a5a`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`external_sub_channel_id`](SaleSource::external_sub_channel_id)."
        )
    )]
    pub external_sub_channel_id: Option<String>,
    /// The sub_channel is an optional field that can be used to provide more
    /// information about the sale_channel. For example, if the sale_channel is
    /// ONLINE_PAYMENTS, the sub_channel could be "WIX" or "QR_CODE" or "WOOCOMMERCE".
    ///
    /// Constraints:
    /// - max length: 50
    ///
    /// Example: `QR_CODE`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`sub_channel`](SaleSource::sub_channel)."
        )
    )]
    pub sub_channel: Option<String>,
}
/// Lifecycle state of the sale.
/// Non-finalised sales are never returned; only fully-paid sales.
///
/// Example: `CLOSED`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SaleStateV10 {
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(untagged)]
    Other(String),
}
/// The currency of the sale.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SalesActivityCurrency {
    /// 3-letter currency code as defined by ISO-4217.
    ///
    /// Constraints:
    /// - format: `iso-4217`
    /// - min length: 3
    /// - max length: 3
    ///
    /// Example: `EUR`
    #[builder(setter(into, doc = "Sets [`code`](SalesActivityCurrency::code)."))]
    pub code: String,
    /// The scaling factor to apply to the value to arrive at the currency's major unit. Providing a scale mitigates changes to the minor unit over time, for example due to inflation. For example, if `scale` is denoted by `s` and `value` is denoted by  `v` is the value, then one arrives at the major currency value by calculating `v / 10^s`.
    ///
    /// Example: `2`
    #[builder(setter(doc = "Sets [`scale`](SalesActivityCurrency::scale)."))]
    pub scale: i64,
}
/// Sale channel filter value. Multi-select uses **OR** within this field.
///
/// Example: `POS`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SalesChannelFilterV10 {
    #[serde(rename = "POS")]
    Pos,
    #[serde(rename = "ONLINE_PAYMENTS")]
    OnlinePayments,
    #[serde(rename = "KIOSK")]
    Kiosk,
    #[serde(rename = "ONLINE_STORE")]
    OnlineStore,
    #[serde(rename = "SOLO")]
    Solo,
    #[serde(rename = "ORDER_AND_PAY")]
    OrderAndPay,
    #[serde(rename = "INVOICE")]
    Invoice,
    #[serde(rename = "BOOKINGS")]
    Bookings,
    #[serde(rename = "ORDER_AGGREGATOR")]
    OrderAggregator,
    #[serde(rename = "SUBSCRIPTIONS")]
    Subscriptions,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(untagged)]
    Other(String),
}
/// Information about tax together with amounts before and after its application such as:
/// - net amount: taxable net amount, scoped by this tax
/// - gross amount: amount after applying the tax.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct TaxLine {
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`gross_amount`](TaxLine::gross_amount)."))]
    pub gross_amount: i64,
    /// Tax ID from the Tax Service (External Id)
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `b064ed44-66e1-45a1-a963-b783808f0436`
    #[builder(setter(into, doc = "Sets [`id`](TaxLine::id)."))]
    pub id: String,
    /// Tax name, e.g. "MwSt."
    ///
    /// Constraints:
    /// - max length: 255
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, into, doc = "Sets [`name`](TaxLine::name).")
    )]
    pub name: Option<String>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`net_amount`](TaxLine::net_amount)."))]
    pub net_amount: i64,
    /// The tax rate percentage in minor units (1/100000). For example 19000 = 19%.
    ///
    /// Constraints:
    /// - value >= 0
    ///
    /// Example: `19000`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(strip_option, doc = "Sets [`percentage`](TaxLine::percentage).")
    )]
    pub percentage: Option<i64>,
    /// The value of the amount in minor units of the currency according to the Sale.currency.scale.
    /// `value` can get large for some currencies and applications.
    /// Use Long type or a similar data type when parsing `value`.
    ///
    /// Example: `100`
    #[builder(setter(doc = "Sets [`tax_amount`](TaxLine::tax_amount)."))]
    pub tax_amount: i64,
}
/// The tax mode used by merchant for displaying prices and calculating taxes:
/// - NO_TAX: The prices don't contain any taxes, merchant don't report taxes
/// - GROSS: The prices used by merchant already include taxes (common in EU), the tax is calculated from the gross price
/// - NET: The prices used by merchant don't include taxes (common in US), the tax is calculated from the net price
/// Since tax values can be different for the same net price depending if the tax
/// was calculated from gross or net price, this info is necessary to be able to reconcile tax calculations.
///
/// Example: `NO_TAX`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TaxMode {
    #[serde(rename = "NO_TAX")]
    NoTax,
    #[serde(rename = "GROSS")]
    Gross,
    #[serde(rename = "NET")]
    Net,
    #[serde(untagged)]
    Other(String),
}
/// The unit of measure of the item.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct UnitOfMeasure {
    /// The group of the unit of measure.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `weight`
    #[builder(setter(into, doc = "Sets [`group_id`](UnitOfMeasure::group_id)."))]
    pub group_id: String,
    /// The specific unit of measure within the group.
    ///
    /// Constraints:
    /// - max length: 100
    ///
    /// Example: `weight.kilogram`
    #[builder(setter(into, doc = "Sets [`unit_id`](UnitOfMeasure::unit_id)."))]
    pub unit_id: String,
    /// The ratio of the specified unit to the base unit of the group.
    ///
    /// Example: `1.24412`
    #[builder(setter(doc = "Sets [`unit_ratio`](UnitOfMeasure::unit_ratio)."))]
    pub unit_ratio: f64,
}
/// The source of the catalog the line originates from.
///
/// Example: `SUMUP`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemReferencesV10CatalogSource {
    #[serde(rename = "SUMUP")]
    Sumup,
    #[serde(rename = "EXTERNAL")]
    External,
    #[serde(untagged)]
    Other(String),
}
/// Credit or debit processing choice, not brand or funding.
/// `unknown` when unavailable; prepaid/voucher are not processing types.
///
/// Example: `debit`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PublicCardV10Type {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(untagged)]
    Other(String),
}
/// The status of the payment.
///
/// Example: `SUCCESSFUL`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PublicPaymentV10Status {
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(untagged)]
    Other(String),
}
/// The type of service for this line.
///
/// Example: `IN_HOUSE`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SaleLineItemV10ServiceType {
    #[serde(rename = "IN_HOUSE")]
    InHouse,
    #[serde(rename = "TAKE_AWAY")]
    TakeAway,
    #[serde(untagged)]
    Other(String),
}
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct ListSalesV10Params {
    /// Pagination token for subsequent requests. Should only be set by clients if they have
    /// received a token in the previous response, and this request is an immediate follow-up
    /// request to that response to load the subsequent page. Clients should treat this as an
    /// opaque token and should not attempt to parse it in any way.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`pagination_token`](ListSalesV10Params::pagination_token)."
        )
    )]
    pub pagination_token: Option<String>,
    /// Maximum number of results per page.
    ///
    /// Constraints:
    /// - value >= 1
    /// - value <= 25
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`pagination_limit`](ListSalesV10Params::pagination_limit)."
        )
    )]
    pub pagination_limit: Option<i64>,
}
/// Filter and pagination criteria for `POST /sales/search`. Filters from different categories
/// stack with **logical AND**; multiple values in one array field stack with **logical OR**.
/// Results cover the merchant’s **last two years** of sales (narrow further with `start_date` /
/// `end_date`). Results are sorted **newest first** by sale close date.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder,
)]
#[builder(doc)]
pub struct SearchSalesV10Request {
    /// Sale created **on or before** this instant (inclusive).
    ///
    /// Example: `2026-06-02T23:59:59Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`end_date`](SearchSalesV10Request::end_date)."
        )
    )]
    pub end_date: Option<crate::datetime::DateTime>,
    /// Maximum number of results per page.
    ///
    /// Constraints:
    /// - value >= 1
    /// - value <= 25
    ///
    /// Example: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`pagination_limit`](SearchSalesV10Request::pagination_limit)."
        )
    )]
    pub pagination_limit: Option<i64>,
    /// Opaque pagination token from a previous search response. Not interchangeable with tokens
    /// from `GET /sales`.
    ///
    /// Example: `66bcbtzelrubzeuzruzgu76678trncH`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            into,
            doc = "Sets [`pagination_token`](SearchSalesV10Request::pagination_token)."
        )
    )]
    pub pagination_token: Option<String>,
    /// Filters rows by payment outcome. Multiple values match any of them (**OR**). The search
    /// row does not echo this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`payment_statuses`](SearchSalesV10Request::payment_statuses)."
        )
    )]
    pub payment_statuses: Option<Vec<PaymentStatusFilterV10>>,
    /// Filters rows by sale lifecycle (`state`). Multiple values match any of them (**OR**).
    /// Uses the same values as `Sale.state`. Non-finalised sales are never returned; only
    /// fully-paid sales.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`sale_statuses`](SearchSalesV10Request::sale_statuses)."
        )
    )]
    pub sale_statuses: Option<Vec<SaleStateV10>>,
    /// Sales channel filter. Multiple values match any of them (**OR**).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`sales_channels`](SearchSalesV10Request::sales_channels)."
        )
    )]
    pub sales_channels: Option<Vec<SalesChannelFilterV10>>,
    /// Sale created **on or after** this instant (inclusive).
    ///
    /// Example: `2026-06-01T00:00:00Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default,
        setter(
            strip_option,
            doc = "Sets [`start_date`](SearchSalesV10Request::start_date)."
        )
    )]
    pub start_date: Option<crate::datetime::DateTime>,
}
use crate::client::Client;
#[derive(Debug, PartialEq)]
pub enum ListSalesV10ErrorBody {
    BadRequest(ProblemV10),
    Unauthorized(ProblemV10),
    Forbidden(ProblemV10),
    PreconditionFailed(ProblemV10),
}
impl crate::error::ProblemDetails for ListSalesV10ErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        None
    }
}
#[derive(Debug, PartialEq)]
pub enum SearchSalesV10ErrorBody {
    BadRequest(ProblemV10),
    Unauthorized(ProblemV10),
    Forbidden(ProblemV10),
    PreconditionFailed(ProblemV10),
}
impl crate::error::ProblemDetails for SearchSalesV10ErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        None
    }
}
#[derive(Debug, PartialEq)]
pub enum GetSaleV10ErrorBody {
    Unauthorized(ProblemV10),
    Forbidden(ProblemV10),
    NotFound(ProblemV10),
    PreconditionFailed(ProblemV10),
}
impl crate::error::ProblemDetails for GetSaleV10ErrorBody {
    fn problem(&self) -> Option<&crate::resources::common::Problem> {
        None
    }
}
/// Client for the Sales API endpoints.
#[derive(Debug)]
pub struct SalesClient<'a> {
    client: &'a Client,
}
impl<'a> SalesClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        self.client
    }
    /// Lists sales
    ///
    /// Returns a paginated list of `Sale` objects. Results are sorted **newest first** by the
    /// **newest event** recorded for each sale. Pagination tokens follow that order.
    ///
    /// Covers the merchant’s **last two years** of sales. Non-finalised sales are never returned;
    /// only fully-paid sales. Use `POST /sales/search` for filtered queries.
    ///
    /// Responses:
    /// - 200: Successful response with a page of sales.
    /// - 400: Validation error on the request.
    /// - 401: The client is not authenticated (auth header is empty or invalid).
    /// - 403: Missing `sales.read`, merchant code mismatch on the token, or no `merchant_read`
    ///   permission.
    /// - 412: User-Agent header is not present or is empty.
    pub async fn list_sales_v10(
        &self,
        merchant_code: impl AsRef<str>,
        params: ListSalesV10Params,
    ) -> crate::error::SdkResult<SalePage, ListSalesV10ErrorBody> {
        let path = format!("/v1.0/merchants/{}/sales", merchant_code.as_ref());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(authorization) = self.client.authorization() {
            request = request.header("Authorization", format!("Bearer {}", authorization));
        }
        for (header_name, header_value) in self.client.runtime_headers() {
            request = request.header(*header_name, header_value);
        }
        if let Some(ref value) = params.pagination_token {
            request = request.query(&[("pagination_token", value)]);
        }
        if let Some(ref value) = params.pagination_limit {
            request = request.query(&[("pagination_limit", value)]);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: SalePage = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw).map(ListSalesV10ErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw)
                        .map(ListSalesV10ErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw).map(ListSalesV10ErrorBody::Forbidden)
                })
                .await)
            }
            reqwest::StatusCode::PRECONDITION_FAILED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw)
                        .map(ListSalesV10ErrorBody::PreconditionFailed)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Searches sales
    ///
    /// Returns a paginated list of `SaleSearchItem` rows matching filter criteria. Results are
    /// sorted **newest first** by **sale close date**.
    ///
    /// Filters from different categories stack with **logical AND**; multiple values in one array
    /// field stack with **logical OR**. Results cover the merchant’s **last two years** of sales
    /// (narrow further with `start_date` / `end_date`). Pagination tokens are not interchangeable
    /// with `GET /sales`.
    ///
    /// Responses:
    /// - 200: Successful response with search results.
    /// - 400: Validation error on the request.
    /// - 401: The client is not authenticated (auth header is empty or invalid).
    /// - 403: Missing `sales.read`, merchant code mismatch on the token, or no `merchant_read`
    ///   permission.
    /// - 412: User-Agent header is not present or is empty.
    pub async fn search_sales_v10(
        &self,
        merchant_code: impl AsRef<str>,
        body: SearchSalesV10Request,
    ) -> crate::error::SdkResult<SaleSearchResponse, SearchSalesV10ErrorBody> {
        let path = format!("/v1.0/merchants/{}/sales/search", merchant_code.as_ref());
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .post(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout())
            .json(&body);
        if let Some(authorization) = self.client.authorization() {
            request = request.header("Authorization", format!("Bearer {}", authorization));
        }
        for (header_name, header_value) in self.client.runtime_headers() {
            request = request.header(*header_name, header_value);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: SaleSearchResponse = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw)
                        .map(SearchSalesV10ErrorBody::BadRequest)
                })
                .await)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw)
                        .map(SearchSalesV10ErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw)
                        .map(SearchSalesV10ErrorBody::Forbidden)
                })
                .await)
            }
            reqwest::StatusCode::PRECONDITION_FAILED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw)
                        .map(SearchSalesV10ErrorBody::PreconditionFailed)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
    /// Gets a sale
    ///
    /// Returns `Sale` for `sale_id`, including item lines and `payments[]`.
    /// Non-finalised sales are never returned; only fully-paid sales.
    ///
    /// Responses:
    /// - 200: Sale data with item lines and payments.
    /// - 401: The client is not authenticated (auth header is empty or invalid).
    /// - 403: Missing `sales.read`, merchant code mismatch on the token, or no `merchant_read`
    ///   permission.
    /// - 404: Sale not found. Non-finalised sales are never returned; only fully-paid sales.
    /// - 412: User-Agent header is not present or is empty.
    pub async fn get_sale_v10(
        &self,
        merchant_code: impl AsRef<str>,
        sale_id: impl AsRef<str>,
    ) -> crate::error::SdkResult<Sale, GetSaleV10ErrorBody> {
        let path = format!(
            "/v1.0/merchants/{}/sales/{}",
            merchant_code.as_ref(),
            sale_id.as_ref()
        );
        let url = format!("{}{}", self.client.base_url(), path);
        let mut request = self
            .client
            .http_client()
            .get(&url)
            .header("User-Agent", crate::version::user_agent())
            .timeout(self.client.timeout());
        if let Some(authorization) = self.client.authorization() {
            request = request.header("Authorization", format!("Bearer {}", authorization));
        }
        for (header_name, header_value) in self.client.runtime_headers() {
            request = request.header(*header_name, header_value);
        }
        let response = request.send().await?;
        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {
                let data: Sale = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw).map(GetSaleV10ErrorBody::Unauthorized)
                })
                .await)
            }
            reqwest::StatusCode::FORBIDDEN => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw).map(GetSaleV10ErrorBody::Forbidden)
                })
                .await)
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw).map(GetSaleV10ErrorBody::NotFound)
                })
                .await)
            }
            reqwest::StatusCode::PRECONDITION_FAILED => {
                Err(crate::error::SdkError::from_response(response, |raw| {
                    serde_json::from_slice::<ProblemV10>(raw)
                        .map(GetSaleV10ErrorBody::PreconditionFailed)
                })
                .await)
            }
            _ => Err(crate::error::SdkError::unexpected_response(response).await),
        }
    }
}
