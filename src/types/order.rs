use std::fmt;

/// Unique identifier for an order
pub type OrderId = u64;

/// Symbol identifier (e.g., "AAPL", "MSFT")
pub type Symbol = String;

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
    pub symbol: Symbol,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Option<Price>,   // We use Option because market orders won't take a price. AKA nullable
    pub quantity: Quantity,
    pub timestamp: u64,
}

// Define methods for Order struct
impl Order {
    /// Create a limit order without ID or timestamp (will be assigned by engine)
    pub fn limit(
        symbol: Symbol,
        side: Side,
        price: Price,
        quantity: Quantity,
    ) -> Self {
        Order {
            id: 0,  // Placeholder, will be set by engine
            symbol,
            side,
            order_type: OrderType::Limit,
            price: Some(price),
            quantity,
            timestamp: 0,  // Will be set by engine
        }
    }

    /// Create a market order without ID or timestamp (will be assigned by engine)
    pub fn market(
        symbol: Symbol,
        side: Side,
        quantity: Quantity,
    ) -> Self {
        Order {
            id: 0,  // Placeholder, will be set by engine
            symbol,
            side,
            order_type: OrderType::Market,
            price: None,
            quantity,
            timestamp: 0,  // Will be set by engine
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

