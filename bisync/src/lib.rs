//! This crate can be used to easily write code for synchronous as well as asynchronous clients.
//!
//! The use should be straight-forward.
//! You can put this into your file where you'd like to have both synchronous and asynchronous submodules:
//!
//! ```
//! // lib.rs
//!
//! #[path = "."]
//! pub mod asynchronous {
//!     use bisync::asynchronous::*;
//!     // If you use the `suffix!` macro, it needs to be in scope:
//!     // use bisync::suffix;
//!     mod inner;
//!     pub use inner::*;
//! }
//!
//! // here you could also add `#[cfg]` attributes to enable or disable this module
//! #[path = "."]
//! pub mod blocking {
//!     use bisync::synchronous::*;
//!     // If you use the `suffix!` macro, it needs to be in scope:
//!     // use bisync::suffix;
//!     mod inner;
//!     pub use inner::*;
//! }
//! ```
//! (The reason why I recommend to copy-paste this instead of providing a macro that does it for you is because LSPs struggle with modules defined within macros)
//!
//! Then, within your `inner` module, you can write code that is async-generic:
//! ```
//! // inner.rs
//!
//! // these are all the available definitions:
//! use super::{bisync, only_sync, only_async, SYNC, ASYNC, suffix};
//!  
//! #[bisync]
//! pub async fn foo() -> String {
//!     bar().await
//! }
//!  
//! #[bisync]
//! async fn bar() -> String {
//!     if ASYNC {
//!         println!("We are in async code.");
//!     } else if SYNC {
//!         println!("We are in blocking code.");
//!     } else {
//!         panic!("This is neither async nor blocking code but a secret third thing.");
//!     }
//!  
//!     baz().await
//!
//!     // Example usage of the suffix macro assuming http_get is the base method
//!     // and your crate has "async" and "blocking" features.
//!     // The suffix macro uses these features for conditional compilation.
//!     let _response = suffix!("_async", http_get("https://example.com").await.unwrap());
//!     // This would expand to:
//!     // #[cfg(feature = "async")] { http_get_async("https://example.com").await.unwrap() }
//!     // #[cfg(all(feature = "blocking", not(feature = "async")))] { http_get("https://example.com").await.unwrap() }
//!     // Then #[bisync]'s internal_strip_async would remove .await for the blocking path.
//! }
//!
//!
//!
//! // Dummy function for example
//! async fn http_get(_url: &str) -> Result<String, ()> { Ok("dummy sync".to_string()) }
//! async fn http_get_async(_url: &str) -> Result<String, ()> { Ok("dummy async".to_string()) }
//!
//! #[only_sync]
//! fn baz() -> String {
//!     ureq::get("https://example.com")
//!         .call()
//!         .unwrap()
//!         .into_string()
//!         .unwrap()
//! }
//!  
//! #[only_async]
//! async fn baz() -> String {
//!     reqwest::get("https://example.com")
//!         .await
//!         .unwrap()
//!         .text()
//!         .await
//!         .unwrap()
//! }
//! ```
//!
//! Here, depending on if we are within async or sync code, we use a different library to perform the requests.
//! As you can see, we prevent duplicate definitions of `foo` and `bar` because they get generated twice,
//! once in synchronous form, and once in asynchronous form.
//!
//! The `suffix!` macro allows conditional compilation of method calls based on features of the *crate using bisync*,
//! typically "async" and "blocking". It suffixes the method call immediately preceding an `.await`.
//!
//! **Using `suffix!` Macro:**
//!
//! The `suffix!` macro expects the *user's crate* to have features like "async" and "blocking" defined.
//! It's called like this: `suffix!("_async", expression_containing_method().await)`.
//!
//! - If `feature = "async"` is enabled: `expression_containing_method_async().await`
//! - If `feature = "blocking"` (and not "async") is enabled: `expression_containing_method().await`
//!
//! The `#[bisync]` attribute will then further process this: `internal_strip_async` will remove the `.await`
//! from the blocking path.

#![no_std]

/// The definitions to use for the synchronous code variation
pub mod synchronous {
    /// Specialize an item to only be emitted in the asynchronous module
    pub use ::bisync_macros::internal_delete as only_async;
    /// Specialize an item to only be emitted in the synchronous module
    pub use ::bisync_macros::internal_noop as only_sync;
    /// Emit an item in both synchronous and asynchronous code, and adjust asynchronisity depending on location
    pub use ::bisync_macros::internal_strip_async as bisync;
    /// true in the synchronous module, otherwise false
    pub const SYNC: bool = true;
    /// true in the asynchronous module, otherwise false
    pub const ASYNC: bool = false;
}

/// The definitions to use for the asynchronous code variation
pub mod asynchronous {
    /// Specialize an item to only be emitted in the synchronous module
    pub use ::bisync_macros::internal_delete as only_sync;
    /// Specialize an item to only be emitted in the asynchronous module
    pub use ::bisync_macros::internal_noop as only_async;
    /// Emit an item in both synchronous and asynchronous code, and adjust asynchronisity depending on location
    pub use ::bisync_macros::internal_noop as bisync;
    /// true in the synchronous module, otherwise false
    pub const SYNC: bool = false;
    /// true in the asynchronous module, otherwise false
    pub const ASYNC: bool = true;
}

// Re-export the `suffix` procedural macro so users can call `bisync::suffix!`
pub use ::bisync_macros::suffix;
