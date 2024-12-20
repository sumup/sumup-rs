/// Example: Create a checkout for a card reader payment
///
/// This example demonstrates how to create a checkout for 10 EUR
/// that can be processed on a SumUp card reader.
///
/// To run this example:
/// 1. Set your API key: export SUMUP_API_KEY="your_api_key_here"
/// 2. Set your merchant code: export SUMUP_MERCHANT_CODE="your_merchant_code"
/// 3. Run: cargo run --example checkout_card_reader
use sumup::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let merchant_code = std::env::var("SUMUP_MERCHANT_CODE")
        .expect("SUMUP_MERCHANT_CODE environment variable must be set");

    let readers = client
        .readers()
        .list(&merchant_code)
        .await
        .expect("couldn't list merchant's readers");

    let reader = readers
        .items
        .first()
        .expect("merchant doesn't have any paired card readers");

    // Generate a unique checkout reference
    let checkout_reference = format!("checkout-{}", uuid::Uuid::new_v4());

    println!("Creating checkout with reference: {}", checkout_reference);

    // Initiate the checkout on a card reader for a card-present payment
    match client
        .readers()
        .create_checkout(
            &merchant_code,
            &reader.id,
            sumup::CreateReaderCheckoutRequest {
                total_amount: sumup::CreateReaderCheckoutRequestTotalAmount {
                    currency: "EUR".into(),
                    minor_unit: 2,
                    value: 1000,
                },
                affiliate: None,
                card_type: None,
                description: Some("sumup-rs card reader checkout example".into()),
                installments: None,
                return_url: None,
                tip_rates: None,
                tip_timeout: None,
            },
        )
        .await
    {
        Ok(_) => {
            println!("✓ Checkout created successfully!");
        }
        Err(e) => {
            eprintln!("✗ Failed to create checkout: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
