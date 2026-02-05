mod types;

use types::{Order, Side};
use types::{Trade};

fn main() {
    println!("Rustex - Low-Latency Order Matching Engine");
    println!("===========================================\n");

    // Create some example orders
    let buy_order = Order::new_limit(1, Side::Buy, 10000, 100, 0);
    let sell_order = Order::new_limit(2, Side::Sell, 10050, 50, 1);
    let market_order = Order::new_market(3, Side::Buy, 25, 2);

    println!("{}", buy_order);
    println!("{}", sell_order);
    println!("{}", market_order);

    let trade = Trade::new(1, 2, 15000, 50, 12345);
    println!("{}", trade);
}