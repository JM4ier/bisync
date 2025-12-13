use super::crate_root::*;

#[only_sync]
pub fn do_stuff() {
    println!("blocking b::do_stuff");
}

#[only_async]
pub async fn do_stuff() {
    println!("async b::do_stuff");
}
