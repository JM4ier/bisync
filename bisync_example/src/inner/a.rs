use super::crate_root::*;

#[bisync]
pub async fn call_b_absolute() {
    crate_root::b::do_stuff().await;
}

#[bisync]
pub async fn call_b_relative() {
    super::b::do_stuff().await;
}
