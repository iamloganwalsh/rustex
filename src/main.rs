mod types;
mod orderbook;
mod matching;

use types::{Order, Side};
use matching::MatchingEngine;

fn main() {
    println!("Rustex - Low-Latency Order Matching Engine");
    println!("===========================================\n");

    let mut engine = MatchingEngine::new();

    // Load the book with both sides
    engine.process_order(Order::new_limit(1, Side::Sell, 15100, 50, 0));
    engine.process_order(Order::new_limit(2, Side::Sell, 15200, 75, 1));
    engine.process_order(Order::new_limit(3, Side::Buy, 14900, 100, 2));

    println!("Book loaded:");
    println!("Best bid: {:?}", engine.order_book.best_bid());
    println!("Best ask: {:?}\n", engine.order_book.best_ask());

    // Buy order that matches
    let buy_order = Order::new_limit(4, Side::Buy, 15150, 80, 3);
    let trades = engine.process_order(buy_order);

    println!("Buy 80 @ $151.50 resulted in {} trades:", trades.len());
    for trade in &trades {
        println!("  {}", trade);
    }

    println!("\nFinal book:");
    println!("Best bid: {:?}", engine.order_book.best_bid());
    println!("Best ask: {:?}", engine.order_book.best_ask());
}