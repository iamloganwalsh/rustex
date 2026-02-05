use std::fmt;

/// Unique identifier for an order
pub type OrderId = u64;

/// Price in the smallest currency unit (e.g., cents)
pub type Price = u64;

/// Quantity of the asset
pub type Quantity = u64;

/// Is the Order Buy or Sell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

/// Type of order (Market, Limit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    /// Market order - executes immediately at best available price
    Market,
    /// Limit order - executes only at specified price or better
    Limit,
}

/// Represents a single order in the system
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Option<Price>,   // We use Option because market orders won't take a price. AKA nullable
    pub quantity: Quantity,
    pub timestamp: u64,
}

// Define methods for Order struct
impl Order {
    /// Create a new limit order
    pub fn new_limit(
        id: OrderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        timestamp: u64,
    ) -> Self {
        Order {
            id,
            side,
            order_type: OrderType::Limit,
            price: Some(price),
            quantity,
            timestamp,
        }
    }

    /// Create a new market order
    pub fn new_market(
        id: OrderId,
        side: Side,
        quantity: Quantity,
        timestamp: u64,
    ) -> Self {
        Order {
            id,
            side,
            order_type: OrderType::Market,
            price: None,
            quantity,
            timestamp,
        }
    }

    /// Check if this is a buy order
    pub fn is_buy(&self) -> bool {
        self.side == Side::Buy
    }

    /// Check if this is a sell order
    pub fn is_sell(&self) -> bool {
        self.side == Side::Sell
    }
}


// Printing function for order details
impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.order_type {
            OrderType::Market => {
                write!(
                    f,
                    "Order #{}: {:?} MARKET {} qty",
                    self.id, self.side, self.quantity
                )
            }
            OrderType::Limit => {
                write!(
                    f,
                    "Order #{}: {:?} LIMIT {} @ {} qty",
                    self.id,
                    self.side,
                    self.price.unwrap(),    // Unwrap is safe here because limit orders must take a price
                    self.quantity
                )
            }
        }
    }
}

