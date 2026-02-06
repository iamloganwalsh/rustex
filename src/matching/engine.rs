use crate::types::{Order, Trade, Side, Price};
use crate::orderbook::OrderBook;

/// Matching engine that processes orders against the order book
pub struct MatchingEngine {
    pub order_book: OrderBook,
    next_trade_id: u64,
}

impl MatchingEngine {
    /// Create a new matching engine
    pub fn new() -> Self {
        MatchingEngine {
            order_book: OrderBook::new(),
            next_trade_id: 1,
        }
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
            self.order_book.add_order(order);
        }

        trades
    }

    fn match_buy_order(&mut self, order: &mut Order, trades: &mut Vec<Trade>) {

        // Get asks in ascending order
        let asks = self.order_book.asks_mut();
        
        let price_levels: Vec<Price> = if let Some(buy_price) = order.price {
            // Limit order: only match asks <= buy_price
            asks.range(..=buy_price)
                .map(|(price, _)| *price)
                .collect()
        } else {
            // Market order: match ALL asks (best prices first)
            asks.keys()
                .copied()
                .collect()
        };

        // Process each price level
        for ask_price in price_levels {
            if order.quantity == 0 {
                break; // Buy order fully filled
            }

            // Get orders at this price level
            if let Some(ask_orders) = asks.get_mut(&ask_price) {
                // Match against orders at this price
                while order.quantity > 0 && !ask_orders.is_empty() {
                    let mut ask_order = ask_orders.pop_front().unwrap();

                    // Determine trade quantity
                    let trade_qty = order.quantity.min(ask_order.quantity);

                    // Get timestamp
                    let timestamp = self.next_trade_id;
                    self.next_trade_id += 1;

                    // Create trade
                    let trade = Trade::new(
                        order.id,
                        ask_order.id,
                        ask_price,  // Trade at ask price
                        trade_qty,
                        timestamp,
                    );
                    trades.push(trade);

                    // Update quantities
                    order.quantity -= trade_qty;
                    ask_order.quantity -= trade_qty;

                    // If ask order still has quantity, put it back
                    if ask_order.quantity > 0 {
                        ask_orders.push_front(ask_order);
                    }
                }

                // Remove price level if empty
                if ask_orders.is_empty() {
                    asks.remove(&ask_price);
                }
            }
        }
    }

    fn match_sell_order(&mut self, order: &mut Order, trades: &mut Vec<Trade>) {
        let bids = self.order_book.bids_mut();
    
        // Collect price levels to process
        let price_levels: Vec<Price> = if let Some(sell_price) = order.price {
            // Limit order: only match bids >= sell_price
            bids.range(sell_price..)
                .map(|(price, _)| *price)
                .rev()
                .collect()
        } else {
            // Market order: match ALL bids (best prices first)
            bids.keys()
                .copied()
                .rev()
                .collect()
        };
    

        // Process each price level
        for bid_price in price_levels {
            if order.quantity == 0 {
                break; // Sell order fully filled
            }

            // Get orders at this price level
            if let Some(bid_orders) = bids.get_mut(&bid_price) {
                // Match against orders at this price
                while order.quantity > 0 && !bid_orders.is_empty() {
                    let mut bid_order = bid_orders.pop_front().unwrap();

                    // Determine trade quantity
                    let trade_qty = order.quantity.min(bid_order.quantity);

                    // Get timestamp BEFORE creating trade
                    let timestamp = self.next_trade_id;
                    self.next_trade_id += 1;

                    // Create trade
                    let trade = Trade::new(
                        bid_order.id,
                        order.id,
                        bid_price,  // Trade at bid price (it was there first)
                        trade_qty,
                        timestamp,  // Use pre-fetched timestamp
                    );
                    trades.push(trade);

                    // Update quantities
                    order.quantity -= trade_qty;
                    bid_order.quantity -= trade_qty;

                    // If bid order still has quantity, put it back
                    if bid_order.quantity > 0 {
                        bid_orders.push_front(bid_order);
                    }
                }

                // Remove price level if empty
                if bid_orders.is_empty() {
                    bids.remove(&bid_price);
                }
            }
        }
    }
}