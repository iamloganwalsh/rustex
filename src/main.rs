mod types;

use types::{Order, Side, OrderBook};

fn main() {
    println!("Rustex - Low-Latency Order Matching Engine");
    println!("===========================================\n");

    let mut book = OrderBook::new();

    // Add some orders
    book.add_order(Order::new_limit(1, Side::Buy, 14900, 100, 0));
    book.add_order(Order::new_limit(2, Side::Buy, 15000, 50, 1));
    book.add_order(Order::new_limit(3, Side::Sell, 15100, 75, 2));
    book.add_order(Order::new_limit(4, Side::Sell, 15200, 100, 3));

    println!("Best bid: {:?}", book.best_bid());
    println!("Best ask: {:?}", book.best_ask());
}