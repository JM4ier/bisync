#[path = "."]
pub mod blocking {
    use bisync::synchronous::*;
    mod inner;
    pub use inner::*;
}

#[path = "."]
pub mod asynchronous {
    use bisync::asynchronous::*;
    mod inner;
    pub use inner::*;
}

#[tokio::main]
async fn main() {
    println!("hello from main");
    blocking::foo();
    asynchronous::foo().await;
}
