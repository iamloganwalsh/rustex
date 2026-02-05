use std::collections::{BTreeMap, VecDeque};
use crate::types::{Order, Price, Side};

/// Order book maintaining bid and ask orders
pub struct OrderBook {
    /// Buy orders (bids), sorted by price descending
    bids: BTreeMap<Price, VecDeque<Order>>,
    
    /// Sell orders (asks), sorted by price ascending
    asks: BTreeMap<Price, VecDeque<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let price = match order.price {
            Some(p) => p,
            None => return, // Market orders don't go in the book
        };

        let book = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        book.entry(price)
            .or_insert_with(VecDeque::new)
            .push_back(order);
    }

    /// Get the highest buy price
    pub fn best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    /// Get the lowest sell price
    pub fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }
}