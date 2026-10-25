use serde_json::json;
use sumup::{Currency, Nullable, checkouts, readers, transactions};

#[test]
fn request_builders_convert_strings_and_omit_optional_fields() {
    let pairing_code = String::from("12345678");
    let request = readers::CreateRequest::builder()
        .name("Front desk")
        .pairing_code(pairing_code)
        .build();

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"name": "Front desk", "pairing_code": "12345678"})
    );
    assert_eq!(
        serde_json::from_value::<readers::CreateRequest>(serde_json::to_value(&request).unwrap())
            .unwrap(),
        request
    );
}

#[test]
fn request_builders_preserve_enums_and_nullable_values() {
    let request = checkouts::CreateRequest::builder()
        .amount(10.0)
        .checkout_reference("order-123")
        .currency(Currency::EUR)
        .merchant_code("M123")
        .description("Coffee")
        .valid_until(Nullable::Null)
        .build();

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "amount": 10.0,
            "checkout_reference": "order-123",
            "currency": "EUR",
            "merchant_code": "M123",
            "description": "Coffee",
            "valid_until": null
        })
    );
}

#[test]
fn nested_builders_preserve_absent_null_and_present_fields() {
    let request = readers::CreateCheckoutRequest::builder()
        .total_amount(
            readers::CreateCheckoutRequestTotalAmount::builder()
                .currency("EUR")
                .minor_unit(2)
                .value(1000)
                .build(),
        )
        .affiliate(Nullable::Null)
        .installments(Nullable::Value(3))
        .build();

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "total_amount": {"currency": "EUR", "minor_unit": 2, "value": 1000},
            "affiliate": null,
            "installments": 3
        })
    );
}

#[test]
fn query_builders_accept_optional_strings_and_numbers() {
    let params = transactions::ListParams::builder()
        .transaction_code("TX123")
        .limit(10)
        .build();

    assert_eq!(
        serde_json::to_value(params).unwrap(),
        json!({"transaction_code": "TX123", "limit": 10})
    );
    assert_eq!(
        transactions::ListParams::builder().build(),
        transactions::ListParams::default()
    );
}

#[test]
fn additional_properties_default_to_empty_and_remain_settable() {
    let empty = readers::CreateCheckoutRequestAffiliateTags::builder().build();
    assert_eq!(serde_json::to_value(empty).unwrap(), json!({}));

    let tags = readers::CreateCheckoutRequestAffiliateTags::builder()
        .additional_properties(std::collections::HashMap::from([(
            "order".to_owned(),
            json!("123"),
        )]))
        .build();
    assert_eq!(serde_json::to_value(tags).unwrap(), json!({"order": "123"}));
}
