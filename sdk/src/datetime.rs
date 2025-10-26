// DateTime support using chrono or jiff based on features
//
// By default, chrono is used. Enable the `jiff` feature (and disable default features) to use jiff instead.

#[cfg(all(feature = "chrono", not(feature = "jiff")))]
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[cfg(all(feature = "jiff", not(feature = "chrono")))]
pub type DateTime = jiff::Timestamp;

// Compile error if both or neither features are enabled
#[cfg(all(feature = "chrono", feature = "jiff"))]
compile_error!("Cannot enable both 'chrono' and 'jiff' features at the same time. Choose one.");

#[cfg(not(any(feature = "chrono", feature = "jiff")))]
compile_error!("Must enable either 'chrono' (default) or 'jiff' feature for datetime support.");

// Re-export the datetime library being used
#[cfg(feature = "chrono")]
pub use chrono;

#[cfg(feature = "jiff")]
pub use jiff;
