use rustex::{MatchingEngine, Order, Side};

#[test]
fn test_basic_buy_sell_match() {
    let mut engine = MatchingEngine::new();
    
    // Add sell order first
    let sell_order = Order::limit("AAPL".to_string(), Side::Sell, 15000, 100);
    match engine.submit_order(sell_order) {
        Ok(trades) => assert!(trades.is_empty(), "First order should not match"),
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Add matching buy order
    let buy_order = Order::limit("AAPL".to_string(), Side::Buy, 15000, 100);
    match engine.submit_order(buy_order) {
        Ok(trades) => {
            assert_eq!(trades.len(), 1, "Should create exactly one trade");
            assert_eq!(trades[0].quantity, 100, "Trade quantity should be 100");
            assert_eq!(trades[0].price, 15000, "Trade price should be 15000");
        }
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Order book should be empty
    if let Some(book) = engine.get_order_book("AAPL") {
        assert!(book.best_bid().is_none(), "No bids should remain");
        assert!(book.best_ask().is_none(), "No asks should remain");
    } else {
        panic!("Order book should exist for AAPL");
    }
}

#[test]
fn test_partial_fill_buyer() {
    let mut engine = MatchingEngine::new();
    
    // Add small sell order
    let sell_order = Order::limit("AAPL".to_string(), Side::Sell, 15000, 50);
    match engine.submit_order(sell_order) {
        Ok(trades) => assert!(trades.is_empty()),
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Add larger buy order (only 50 should match)
    let buy_order = Order::limit("AAPL".to_string(), Side::Buy, 15000, 150);
    match engine.submit_order(buy_order) {
        Ok(trades) => {
            assert_eq!(trades.len(), 1, "Should create exactly one trade");
            assert_eq!(trades[0].quantity, 50, "Should only match available 50 shares");
        }
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Check remaining buy order is in book
    if let Some(book) = engine.get_order_book("AAPL") {
        assert_eq!(book.best_bid(), Some(15000), "Remaining buy should be at 15000");
        assert!(book.best_ask().is_none(), "Sell order should be fully filled");
    } else {
        panic!("Order book should exist");
    }
}

#[test]
fn test_partial_fill_seller() {
    let mut engine = MatchingEngine::new();
    
    // Add large buy order
    let buy_order = Order::limit("AAPL".to_string(), Side::Buy, 15000, 200);
    match engine.submit_order(buy_order) {
        Ok(trades) => assert!(trades.is_empty()),
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Add smaller sell order
    let sell_order = Order::limit("AAPL".to_string(), Side::Sell, 15000, 75);
    match engine.submit_order(sell_order) {
        Ok(trades) => {
            assert_eq!(trades.len(), 1, "Should create exactly one trade");
            assert_eq!(trades[0].quantity, 75, "Should match full sell order");
        }
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Check remaining buy order
    if let Some(book) = engine.get_order_book("AAPL") {
        assert_eq!(book.best_bid(), Some(15000), "Partial buy should remain");
        assert!(book.best_ask().is_none(), "Sell should be fully filled");
    } else {
        panic!("Order book should exist");
    }
}

#[test]
fn test_price_time_priority_fifo() {
    let mut engine = MatchingEngine::new();
    
    // Add three buy orders at same price (should match in FIFO order)
    let buy1 = Order::limit("AAPL".to_string(), Side::Buy, 15000, 30);
    let buy2 = Order::limit("AAPL".to_string(), Side::Buy, 15000, 40);
    let buy3 = Order::limit("AAPL".to_string(), Side::Buy, 15000, 50);
    
    match engine.submit_order(buy1) {
        Ok(_) => {},
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    let buy1_id = 1; // First order gets ID 1
    
    match engine.submit_order(buy2) {
        Ok(_) => {},
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    match engine.submit_order(buy3) {
        Ok(_) => {},
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Add sell order that matches first two
    let sell_order = Order::limit("AAPL".to_string(), Side::Sell, 15000, 60);
    match engine.submit_order(sell_order) {
        Ok(trades) => {
            assert_eq!(trades.len(), 2, "Should match with first two buy orders");
            // First trade should be with buy1 (30 shares)
            assert_eq!(trades[0].quantity, 30, "First trade should be 30 shares");
            assert_eq!(trades[0].buyer_order_id, buy1_id, "Should match with first buy order");
            // Second trade should be with buy2 (30 of 40 shares)
            assert_eq!(trades[1].quantity, 30, "Second trade should be 30 shares");
        }
        Err(e) => panic!("Valid order rejected: {}", e),
    }
}

#[test]
fn test_no_match_when_prices_dont_cross() {
    let mut engine = MatchingEngine::new();
    
    // Add buy at 14900
    let buy_order = Order::limit("AAPL".to_string(), Side::Buy, 14900, 100);
    match engine.submit_order(buy_order) {
        Ok(trades) => assert!(trades.is_empty()),
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Add sell at 15100 (spread of 200)
    let sell_order = Order::limit("AAPL".to_string(), Side::Sell, 15100, 100);
    match engine.submit_order(sell_order) {
        Ok(trades) => {
            assert!(trades.is_empty(), "Orders should not match - prices don't cross");
        }
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Both orders should remain in book
    if let Some(book) = engine.get_order_book("AAPL") {
        assert_eq!(book.best_bid(), Some(14900), "Buy order should remain");
        assert_eq!(book.best_ask(), Some(15100), "Sell order should remain");
    } else {
        panic!("Order book should exist");
    }
}

#[test]
fn test_aggressive_order_matches_multiple_levels() {
    let mut engine = MatchingEngine::new();
    
    // Add multiple sell orders at different prices
    let sell1 = Order::limit("AAPL".to_string(), Side::Sell, 15000, 50);
    let sell2 = Order::limit("AAPL".to_string(), Side::Sell, 15100, 75);
    let sell3 = Order::limit("AAPL".to_string(), Side::Sell, 15200, 100);
    
    engine.submit_order(sell1).unwrap();
    engine.submit_order(sell2).unwrap();
    engine.submit_order(sell3).unwrap();
    
    // Aggressive buy sweeps first two levels
    let buy_order = Order::limit("AAPL".to_string(), Side::Buy, 15150, 120);
    match engine.submit_order(buy_order) {
        Ok(trades) => {
            assert_eq!(trades.len(), 2, "Should match across two price levels");
            assert_eq!(trades[0].price, 15000, "First match at best ask");
            assert_eq!(trades[0].quantity, 50, "Match full first level");
            assert_eq!(trades[1].price, 15100, "Second match at next level");
            assert_eq!(trades[1].quantity, 70, "Match partial second level");
        }
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Check remaining orders
    if let Some(book) = engine.get_order_book("AAPL") {
        assert!(book.best_bid().is_none(), "No buy orders should remain");
        assert_eq!(book.best_ask(), Some(15100), "Partial sell at 15100 remains");
    }
}

#[test]
fn test_market_order_sweeps_book() {
    let mut engine = MatchingEngine::new();
    
    // Add multiple sell orders
    let sell1 = Order::limit("AAPL".to_string(), Side::Sell, 15000, 50);
    let sell2 = Order::limit("AAPL".to_string(), Side::Sell, 15100, 75);
    
    engine.submit_order(sell1).unwrap();
    engine.submit_order(sell2).unwrap();
    
    // Market buy should match all available
    let market_buy = Order::market("AAPL".to_string(), Side::Buy, 100);
    match engine.submit_order(market_buy) {
        Ok(trades) => {
            assert_eq!(trades.len(), 2, "Should match both levels");
            assert_eq!(trades[0].quantity, 50, "Match first level");
            assert_eq!(trades[1].quantity, 50, "Match part of second level");
        }
        Err(e) => panic!("Valid order rejected: {}", e),
    }
}

#[test]
fn test_multi_symbol_isolation() {
    let mut engine = MatchingEngine::new();
    
    // Add AAPL buy order
    let aapl_buy = Order::limit("AAPL".to_string(), Side::Buy, 15000, 100);
    engine.submit_order(aapl_buy).unwrap();
    
    // Add MSFT sell order at same price
    let msft_sell = Order::limit("MSFT".to_string(), Side::Sell, 15000, 100);
    match engine.submit_order(msft_sell) {
        Ok(trades) => {
            assert!(trades.is_empty(), "Different symbols should not match");
        }
        Err(e) => panic!("Valid order rejected: {}", e),
    }
    
    // Verify both orders in their respective books
    if let Some(aapl_book) = engine.get_order_book("AAPL") {
        assert_eq!(aapl_book.best_bid(), Some(15000));
        assert!(aapl_book.best_ask().is_none());
    }
    
    if let Some(msft_book) = engine.get_order_book("MSFT") {
        assert!(msft_book.best_bid().is_none());
        assert_eq!(msft_book.best_ask(), Some(15000));
    }
}