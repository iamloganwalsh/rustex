mod types;
use types::order::{Order, Side};

fn main() {
    let order = Order::new_limit(1, Side::Buy, 10000, 100, 0);
    println!("Hello, world!");
    println!("{}", order);
    println!("{:?}", order); 
}
