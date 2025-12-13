#[path = "."]
pub mod blocking {
    use crate::blocking as crate_root;
    use bisync::synchronous::*;
    mod inner;
    pub use inner::*;
}

#[path = "."]
pub mod asynchronous {
    use crate::asynchronous as crate_root;
    use bisync::asynchronous::*;
    mod inner;
    pub use inner::*;
}

#[tokio::main]
async fn main() {
    println!("hello from main");
    blocking::foo();
    asynchronous::foo().await;
    blocking::a::call_b_absolute();
    blocking::a::call_b_relative();
    asynchronous::a::call_b_absolute().await;
    asynchronous::a::call_b_relative().await;
}
