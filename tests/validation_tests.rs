use rustex::{MatchingEngine, Order, Side};

#[test]
fn test_reject_zero_quantity() {
    let mut engine = MatchingEngine::new();

    let limit_order = Order::limit("AAPL".to_string(), Side::Buy, 15000, 0);

    match engine.submit_order(limit_order) {
        Err(msg) => assert!(msg.contains("Invalid order: Quantity must be greater than 0")),
        Ok(_) => panic!("Should have rejected zero quantity"),
    }

    let market_order = Order::market("AAPL".to_string(), Side::Buy, 0);
    
    match engine.submit_order(market_order) {
        Err(msg) => assert!(msg.contains("Invalid order: Quantity must be greater than 0")),
        Ok(_) => panic!("Should have rejected zero quantity"),
    }
}

#[test] // Only needs to test limit orders (market orders don't take price)
fn test_reject_zero_price() {
    let mut engine = MatchingEngine::new();

    let limit_order = Order::limit("AAPL".to_string(), Side::Buy, 0, 15000);
    
    match engine.submit_order(limit_order) {
        Err(msg) => assert!(msg.contains("Invalid order: Price must be greater than 0")),
        Ok(_) => panic!("Should have rejected zero price"),
    }

    let mut limit_order_null_price = Order::limit("AAPL".to_string(), Side::Buy, 0, 15000);
    limit_order_null_price.price = None;

    match engine.submit_order(limit_order_null_price) {
        Err(msg) => assert!(msg.contains("Invalid order: Limit order requires price")),
        Ok(_) => panic!("Should have rejected null price"),
    }
}

#[test]
fn test_reject_empty_symbol() {
    let mut engine = MatchingEngine::new();

    let limit_order = Order::limit("".to_string(), Side::Buy, 1500, 1500);

    match engine.submit_order(limit_order) {
        Err(msg) => assert!(msg.contains("Invalid order: Symbol cannot be empty")),
        Ok(_) => panic!("Should have rejected invalid symbol"),
    }

    let market_order = Order::market("".to_string(), Side::Buy, 1500);
    
    match engine.submit_order(market_order) {
        Err(msg) => assert!(msg.contains("Invalid order: Symbol cannot be empty")),
        Ok(_) => panic!("Should have rejected invalid symbol"),
    }
}

#[test]
fn test_valid_orders() {
    let mut engine = MatchingEngine::new();

    let limit_order = Order::limit("AAPL".to_string(), Side::Buy, 150, 150);

    match engine.submit_order(limit_order) {
        Ok(trades) => assert!(trades.is_empty()),
        Err(e) => panic!("Order should be valid but failed: {e}"),
    }

    let market_order = Order::market("AAPL".to_string(), Side::Buy, 150);

    match engine.submit_order(market_order) {
        Ok(trades) => assert!(trades.is_empty()),
        Err(e) => panic!("Order should be valid but failed: {e}"),
    }
}