use rustex::{MatchingEngine, Order, Side};

#[test]
fn test_cancel_nonexistent_order() {
    let mut engine = MatchingEngine::new();
    
    let cancelled = engine.cancel_order("AAPL", 999);
    assert!(!cancelled, "Should not cancel non-existent order");
}

#[test]
fn test_cancel_from_wrong_symbol() {
    let mut engine = MatchingEngine::new();
    
    // Add order to AAPL
    let order = Order::limit("AAPL".to_string(), Side::Buy, 15000, 100);
    engine.submit_order(order).unwrap();
    
    // Try to cancel from MSFT
    let cancelled = engine.cancel_order("MSFT", 1);
    assert!(!cancelled, "Should not find order in wrong symbol");
    
    // Verify order still exists in AAPL
    if let Some(book) = engine.get_order_book("AAPL") {
        assert_eq!(book.best_bid(), Some(15000));
    }
}

#[test]
fn test_cancel_preserves_other_orders() {
    let mut engine = MatchingEngine::new();
    
    // Add 3 orders at same price
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 100)).unwrap();
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 50)).unwrap();
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 75)).unwrap();
    
    // Cancel middle order
    let cancelled = engine.cancel_order("AAPL", 2);
    assert!(cancelled);
    
    // Other orders should remain
    if let Some(book) = engine.get_order_book("AAPL") {
        assert_eq!(book.best_bid(), Some(15000));
    }
}

#[test]
fn test_cancel_removes_price_level_when_empty() {
    let mut engine = MatchingEngine::new();
    
    // Add single order at a price
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 100)).unwrap();
    
    // Verify order exists
    if let Some(book) = engine.get_order_book("AAPL") {
        assert_eq!(book.best_bid(), Some(15000));
    }
    
    // Cancel the order
    engine.cancel_order("AAPL", 1);
    
    // Price level should be removed
    if let Some(book) = engine.get_order_book("AAPL") {
        assert!(book.best_bid().is_none(), "Price level should be removed");
    }
}

#[test]
fn test_cancel_multiple_orders_sequentially() {
    let mut engine = MatchingEngine::new();
    
    // Add 5 orders
    for _ in 0..5 {
        engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 100)).unwrap();
    }
    
    // Cancel them one by one
    for id in 1..=5 {
        let cancelled = engine.cancel_order("AAPL", id);
        assert!(cancelled, "Should cancel order {}", id);
    }
    
    // Book should be empty
    if let Some(book) = engine.get_order_book("AAPL") {
        assert!(book.best_bid().is_none());
        assert!(book.best_ask().is_none());
    }
}

#[test]
fn test_cancel_after_partial_fill() {
    let mut engine = MatchingEngine::new();
    
    // Add large buy order
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 200)).unwrap();
    
    // Partially fill it
    let sell = Order::limit("AAPL".to_string(), Side::Sell, 15000, 75);
    let trades = engine.submit_order(sell).unwrap();
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, 75);
    
    // Cancel remaining portion
    let cancelled = engine.cancel_order("AAPL", 1);
    assert!(cancelled, "Should cancel partially filled order");
    
    // Book should be empty
    if let Some(book) = engine.get_order_book("AAPL") {
        assert!(book.best_bid().is_none());
    }
}

#[test]
fn test_cannot_cancel_fully_filled_order() {
    let mut engine = MatchingEngine::new();
    
    // Add buy order
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 100)).unwrap();
    
    // Fully fill it
    let sell = Order::limit("AAPL".to_string(), Side::Sell, 15000, 100);
    let trades = engine.submit_order(sell).unwrap();
    assert_eq!(trades[0].quantity, 100);
    
    // Try to cancel (should fail - order is gone)
    let cancelled = engine.cancel_order("AAPL", 1);
    assert!(!cancelled, "Cannot cancel fully filled order");
}

#[test]
fn test_cancel_maintains_price_time_priority() {
    let mut engine = MatchingEngine::new();
    
    // Add 3 orders at same price
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 30)).unwrap(); // ID 1
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 40)).unwrap(); // ID 2
    engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, 15000, 50)).unwrap(); // ID 3
    
    // Cancel first order
    engine.cancel_order("AAPL", 1);
    
    // Now match - should match with order #2 (now first in line)
    let sell = Order::limit("AAPL".to_string(), Side::Sell, 15000, 35);
    let trades = engine.submit_order(sell).unwrap();
    
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].buyer_order_id, 2, "Should match with order #2");
    assert_eq!(trades[0].quantity, 35);
}