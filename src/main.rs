mod types;
mod orderbook;
mod matching;

use types::{Order, Side};
use matching::MatchingEngine;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  Rustex - Low-Latency Order Matching Engine");
    println!("═══════════════════════════════════════════════════════\n");

    let mut engine = MatchingEngine::new();
    let mut order_id = 1u64;

    // Scenario 1: Build initial book
    println!("SCENARIO 1: Building initial order book\n");
    
    engine.process_order(Order::new_limit(order_id, Side::Buy, 14900, 100, 0));
    order_id += 1;
    engine.process_order(Order::new_limit(order_id, Side::Buy, 15000, 150, 1));
    order_id += 1;
    engine.process_order(Order::new_limit(order_id, Side::Buy, 15000, 50, 2));
    order_id += 1;
    
    engine.process_order(Order::new_limit(order_id, Side::Sell, 15100, 75, 3));
    order_id += 1;
    engine.process_order(Order::new_limit(order_id, Side::Sell, 15200, 100, 4));
    order_id += 1;
    engine.process_order(Order::new_limit(order_id, Side::Sell, 15300, 125, 5));
    order_id += 1;

    engine.order_book.display();
    println!("Spread: ${:.2}\n", 
        (engine.order_book.best_ask().unwrap() - engine.order_book.best_bid().unwrap()) as f64 / 100.0);

    // Scenario 2: Limit order crosses spread
    println!("═══════════════════════════════════════════════════════");
    println!("SCENARIO 2: Aggressive buy order crosses spread\n");
    println!("New order: Buy 120 @ $152.00 (limit)");
    
    let trades = engine.process_order(Order::new_limit(order_id, Side::Buy, 15200, 120, 6));
    order_id += 1;
    
    println!("\nTrades executed: {}", trades.len());
    for trade in &trades {
        println!("   {}", trade);
    }
    
    engine.order_book.display();

    // Scenario 3: Market order
    println!("═══════════════════════════════════════════════════════");
    println!("🎯 SCENARIO 3: Market order sweeps book\n");
    println!("New order: Sell 250 (market)");
    
    let trades = engine.process_order(Order::new_market(order_id, Side::Sell, 250, 7));
    order_id += 1;
    
    println!("\nTrades executed: {}", trades.len());
    for trade in &trades {
        println!("   {}", trade);
    }
    
    engine.order_book.display();

    // Scenario 4: Order cancellation
    println!("═══════════════════════════════════════════════════════");
    println!("SCENARIO 4: Cancel an order\n");
    
    // Add an order we'll cancel
    engine.process_order(Order::new_limit(order_id, Side::Sell, 15500, 200, 8));
    let cancel_id = order_id;
    order_id += 1;
    
    println!("Added order #{} (Sell 200 @ $155.00)", cancel_id);
    engine.order_book.display();
    
    println!("Cancelling order #{}...", cancel_id);
    let cancelled = engine.order_book.cancel_order(cancel_id);
    println!("Cancellation {}\n", if cancelled { "successful" } else { "failed" });
    
    engine.order_book.display();

    // Scenario 5: Partial fill
    println!("═══════════════════════════════════════════════════════");
    println!("SCENARIO 5: Partial fill\n");
    
    // Add large sell order
    engine.process_order(Order::new_limit(order_id, Side::Sell, 15400, 500, 9));
    order_id += 1;
    
    println!("New order: Buy 100 @ $154.00 (limit)");
    let trades = engine.process_order(Order::new_limit(order_id, Side::Buy, 15400, 100, 10));
    
    println!("\nTrades executed: {}", trades.len());
    for trade in &trades {
        println!("   {}", trade);
    }
    println!("\n(Original sell order had 500 shares, 100 filled, 400 remain)");
    
    engine.order_book.display();

    // Final summary
    println!("═══════════════════════════════════════════════════════");
    println!("FINAL SUMMARY\n");
    println!("Best bid: ${:.2}", engine.order_book.best_bid().unwrap_or(0) as f64 / 100.0);
    println!("Best ask: ${:.2}", engine.order_book.best_ask().unwrap_or(0) as f64 / 100.0);
    if let (Some(bid), Some(ask)) = (engine.order_book.best_bid(), engine.order_book.best_ask()) {
        println!("Spread: ${:.2}", (ask - bid) as f64 / 100.0);
    }
    println!("\n═══════════════════════════════════════════════════════");
}