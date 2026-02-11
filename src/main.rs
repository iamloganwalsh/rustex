mod types;
mod orderbook;
mod matching;

use types::{Order, Side};
use matching::MatchingEngine;

fn main() {
    let mut engine = MatchingEngine::new();
    let symbol = "AAPL".to_string();

    let orders = vec![
        // Initial liquidity
        Order::limit(symbol.clone(), Side::Buy, 14900, 100),
        Order::limit(symbol.clone(), Side::Buy, 15000, 150),
        Order::limit(symbol.clone(), Side::Sell, 15100, 75),
        Order::limit(symbol.clone(), Side::Sell, 15200, 200),

        // Crossing orders (these should match)
        Order::limit(symbol.clone(), Side::Buy, 15100, 50),   // matches 15100 sell
        Order::limit(symbol.clone(), Side::Sell, 15000, 100), // matches 15000 buy

        // Market order
        Order::market(symbol.clone(), Side::Buy, 120),

        // Invalid Orders
        Order::limit("".to_string(), Side::Buy, 100, 200),  // Invalid symbol
        Order::limit(symbol.clone(), Side::Buy, 0, 200),    // Invalid price
        Order::limit(symbol.clone(), Side::Buy, 100, 0),    // Invalid quantity
    ];

    for order in orders {
        match engine.submit_order(order) {
            Ok(trades) => {
                if !trades.is_empty() {
                    println!("Trades executed:");
                    for trade in trades {
                        println!("{:?}", trade);
                    }
                }
            }
            Err(e) => {
                eprintln!("Order rejected: {e}");
            }
        }
    }

    println!("\nFinal Order Book:");
    if let Some(book) = engine.get_order_book("AAPL") {
        book.display();
    }
}
