use super::{bisync, only_sync, only_async, SYNC, ASYNC}; // all the generated definitions
 
#[bisync]
pub async fn foo() -> String {
    bar().await
}
 
#[bisync]
async fn bar() -> String {
    if ASYNC {
        println!("We are in async code.");
    } else if SYNC {
        println!("We are in blocking code.");
    } else {
        panic!("This is neither async nor blocking code but a secret third thing.");
    }
 
    baz().await
}
 
#[only_sync]
fn baz() -> String {
    ureq::get("https://example.com")
        .call()
        .unwrap()
        .into_string()
        .unwrap()
}
 
#[only_async]
async fn baz() -> String {
    reqwest::get("https://example.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
