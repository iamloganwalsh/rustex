use std::collections::{BTreeMap, VecDeque, HashMap};
use crate::types::{Order, OrderId, Price, Side};

/// Order book maintaining bid and ask orders
pub struct OrderBook {
    /// Buy orders (bids), sorted by price descending
    bids: BTreeMap<Price, VecDeque<Order>>,
    
    /// Sell orders (asks), sorted by price ascending
    asks: BTreeMap<Price, VecDeque<Order>>,

    /// Store order locations for O(1) lookup by Order ID
    order_locations: HashMap<OrderId, (Side, Price)>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_locations: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let price = match order.price {
            Some(p) => p,
            None => return,
        };
    
        let side = order.side;
        let order_id = order.id;  // capture ID before moving order
    
        let book = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
    
        book.entry(price)
            .or_insert_with(VecDeque::new)
            .push_back(order);
        
        // Track order location
        self.order_locations.insert(order_id, (side, price));
    }

    pub fn cancel_order(&mut self, order_id: OrderId) -> bool {
        // Look up order location
        let (side, price) = match self.order_locations.remove(&order_id) {
            Some(loc) => loc,
            None => return false, // Order not found
        };
    
        // Get the appropriate book
        let book = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
    
        // Get orders at this price level
        if let Some(orders) = book.get_mut(&price) {
            // Find and remove the order
            if let Some(pos) = orders.iter().position(|o| o.id == order_id) {
                orders.remove(pos);
                
                // Remove price level if empty
                if orders.is_empty() {
                    book.remove(&price);
                }
                
                return true;
            }
        }
    
        false
    }

    /// Get the highest buy price
    pub fn best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    /// Get the lowest sell price
    pub fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    /// Get mutable reference to bids
    pub fn bids_mut(&mut self) -> &mut BTreeMap<Price, VecDeque<Order>> {
        &mut self.bids
    }

    /// Get mutable reference to asks
    pub fn asks_mut(&mut self) -> &mut BTreeMap<Price, VecDeque<Order>> {
        &mut self.asks
    }

    pub fn display(&self) {
        println!("\nORDER BOOK");
        println!("==========");
        
        // Display asks (lowest first, so reverse)
        println!("ASKS (Sells):");
        let mut ask_levels: Vec<_> = self.asks.iter().collect();
        ask_levels.reverse();
        
        for (price, orders) in ask_levels {
            let total_qty: u64 = orders.iter().map(|o| o.quantity).sum();
            let num_orders = orders.len();
            println!(
                "  ${:>7.2}  |  {:>4} shares  ({} order{})",
                *price as f64 / 100.0,
                total_qty,
                num_orders,
                if num_orders == 1 { "" } else { "s" }
            );
        }
        
        // Separator
        println!("  ─────────────────────────────────");
        
        // Display bids (highest first)
        println!("BIDS (Buys):");
        for (price, orders) in self.bids.iter().rev() {
            let total_qty: u64 = orders.iter().map(|o| o.quantity).sum();
            let num_orders = orders.len();
            println!(
                "  ${:>7.2}  |  {:>4} shares  ({} order{})",
                *price as f64 / 100.0,
                total_qty,
                num_orders,
                if num_orders == 1 { "" } else { "s" }
            );
        }
        println!();  // Extra newline at end
    }
}