use crate::types::{Order, OrderType, Symbol, Trade, Side, Price};
use crate::orderbook::OrderBook;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MatchingEngine {
    order_books: HashMap<Symbol, OrderBook>,  // One book per symbol
    next_trade_id: u64,
    next_order_id: u64,
}

impl MatchingEngine {
    /// Create a new matching engine
    pub fn new() -> Self {
        MatchingEngine {
            order_books: HashMap::new(),
            next_trade_id: 1,
            next_order_id: 1,
        }
    }

    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    pub fn submit_order(&mut self, mut order: Order) -> Result<Vec<Trade>, String> {

        // Validate order
        Self::validate_order(&order)?;   // if Err, returns early
    
        // Assign ID and timestamp
        order.id = self.next_order_id;
        self.next_order_id += 1;
        order.timestamp = Self::get_timestamp();
    
        // Process
        Ok(self.process_order(order))
    }
    
    fn validate_order(order: &Order) -> Result<(), String> {
        if order.quantity == 0 {
            return Err(format!(
                "Invalid order: Quantity must be greater than 0; (symbol={}, side={:?}, price={:?}, qty={:?})",
                order.symbol,
                order.side,
                order.price,
                order.quantity
            ));
        }
    
        if order.symbol.is_empty() {
            return Err(format!(
                "Invalid order: Symbol cannot be empty; (symbol={}, side={:?}, price={:?}, qty={})",
                order.symbol,
                order.side,
                order.price,
                order.quantity
            ));
        }
    
        if order.order_type == OrderType::Limit {
            // If price ever becomes None since it's Some()
            let price = order.price.ok_or_else(|| {
                format!(
                    "Invalid order: Limit order requires price; (symbol={}, side={:?}, price={:?}, qty={})",
                    order.symbol,
                    order.side,
                    order.price,
                    order.quantity
                )
            })?;
    
            if price == 0 {
                return Err(format!(
                    "Invalid order: Price must be greater than 0; (symbol={}, side={:?}, price={:?}, qty={})",
                    order.symbol,
                    order.side,
                    order.price,
                    order.quantity
                ));
            }
        }
    
        Ok(())
    }
    
    /// Init or retrieve book for symbol
    fn get_or_create_book(&mut self, symbol: &str) -> &mut OrderBook {
        self.order_books
            .entry(symbol.to_string())
            .or_insert_with(OrderBook::new)
    }

    /// Get read-only order book for a symbol
    pub fn get_order_book(&self, symbol: &str) -> Option<&OrderBook> {
        self.order_books.get(symbol)
    }

    /// Process an incoming order
    pub fn process_order(&mut self, mut order: Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        // Try to match the order
        match order.side {
            Side::Buy => self.match_buy_order(&mut order, &mut trades),
            Side::Sell => self.match_sell_order(&mut order, &mut trades),
        }

        // If order has remaining quantity, add to book
        if order.quantity > 0 {
            let book = self.get_or_create_book(&order.symbol);
            book.add_order(order);
        }

        trades
    }

    fn match_buy_order(&mut self, order: &mut Order, trades: &mut Vec<Trade>) {
        // Get price levels first (before we borrow the book)
        let symbol = order.symbol.clone();
        
        let price_levels: Vec<Price> = {
            let book = self.get_or_create_book(&symbol);
            let asks = book.asks_mut();
            
            if let Some(buy_price) = order.price {
                asks.range(..=buy_price)
                    .map(|(price, _)| *price)
                    .collect()
            } else {
                asks.keys()
                    .copied()
                    .collect()
            }
        }; // book borrow ends here
    
        // Now process without holding the book borrow
        for ask_price in price_levels {
            if order.quantity == 0 {
                break;
            }
    
            // Get asks for each iteration
            let asks = {
                let book = self.order_books
                    .entry(symbol.clone())
                    .or_insert_with(OrderBook::new);
                book.asks_mut()
            };            
    
            if let Some(ask_orders) = asks.get_mut(&ask_price) {
                while order.quantity > 0 && !ask_orders.is_empty() {
                    let mut ask_order = ask_orders.pop_front().unwrap();
                    let trade_qty = order.quantity.min(ask_order.quantity);
    
                    // Retrieve timestamp
                    let timestamp = MatchingEngine::get_timestamp();
    
                    let trade = Trade::new(
                        order.id,
                        ask_order.id,
                        ask_price,
                        trade_qty,
                        timestamp,
                    );
                    trades.push(trade);
    
                    order.quantity -= trade_qty;
                    ask_order.quantity -= trade_qty;
    
                    // Re-borrow to put order back if needed
                    if ask_order.quantity > 0 {
                        let book = self.get_or_create_book(&symbol);
                        let asks = book.asks_mut();
                        if let Some(orders) = asks.get_mut(&ask_price) {
                            orders.push_front(ask_order);
                        }
                        break; // Exit inner loop since we put order back
                    }
                }
    
                // Clean up empty price levels
                let book = self.get_or_create_book(&symbol);
                let asks = book.asks_mut();
                if let Some(orders) = asks.get(&ask_price) {
                    if orders.is_empty() {
                        asks.remove(&ask_price);
                    }
                }
            }
        }
    }
    
    fn match_sell_order(&mut self, order: &mut Order, trades: &mut Vec<Trade>) {
        // Get price levels first (before we borrow the book)
        let symbol = order.symbol.clone();
        
        let price_levels: Vec<Price> = {
            let book = self.get_or_create_book(&symbol);
            let bids = book.bids_mut();
            
            if let Some(sell_price) = order.price {
                bids.range(sell_price..)
                    .map(|(price, _)| *price)
                    .rev()
                    .collect()
            } else {
                bids.keys()
                    .copied()
                    .rev()
                    .collect()
            }
        }; // book borrow ends here
    
        // Now process without holding the book borrow
        for bid_price in price_levels {
            if order.quantity == 0 {
                break;
            }
    
            // Get bids for each iteration
            let bids = {
                let book = self.order_books
                    .entry(symbol.clone())
                    .or_insert_with(OrderBook::new);
                book.bids_mut()
            };
    
            if let Some(bid_orders) = bids.get_mut(&bid_price) {
                while order.quantity > 0 && !bid_orders.is_empty() {
                    let mut bid_order = bid_orders.pop_front().unwrap();
                    let trade_qty = order.quantity.min(bid_order.quantity);
    
                    // Retrieve timestamp
                    let timestamp = MatchingEngine::get_timestamp();
    
                    let trade = Trade::new(
                        bid_order.id,
                        order.id,
                        bid_price,
                        trade_qty,
                        timestamp,
                    );
                    trades.push(trade);
    
                    order.quantity -= trade_qty;
                    bid_order.quantity -= trade_qty;
    
                    // Re-borrow to put order back if needed
                    if bid_order.quantity > 0 {
                        let book = self.get_or_create_book(&symbol);
                        let bids = book.bids_mut();
                        if let Some(orders) = bids.get_mut(&bid_price) {
                            orders.push_front(bid_order);
                        }
                        break; // Exit inner loop since we put order back
                    }
                }
    
                // Clean up empty price levels
                let book = self.get_or_create_book(&symbol);
                let bids = book.bids_mut();
                if let Some(orders) = bids.get(&bid_price) {
                    if orders.is_empty() {
                        bids.remove(&bid_price);
                    }
                }
            }
        }
    }
}