use super::{OrderId, Price, Quantity};
use std::fmt;

/// Represents a trade that occurred when two orders matched
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    pub buyer_order_id: OrderId,
    pub seller_order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
    pub timestamp: u64,
}

impl Trade {
    /// Create a new trade
    pub fn new(
        buyer_order_id: OrderId,
        seller_order_id: OrderId,
        price: Price,
        quantity: Quantity,
        timestamp: u64,
    ) -> Self {
        Trade {
            buyer_order_id,
            seller_order_id,
            price,
            quantity,
            timestamp,
        }
    }
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trade: {} shares @ {} (Buy Order #{}, Sell Order #{})",
            self.quantity, self.price, self.buyer_order_id, self.seller_order_id
        )
    }
}