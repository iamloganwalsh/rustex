# Rustex - Low-Latency Order Matching Engine

A low-latency order matching engine built in Rust, implementing price-time priority matching with multi-symbol support.

## Overview

Rustex is a production-quality matching engine that processes limit and market orders with sub-microsecond latency. It demonstrates advanced systems programming concepts including:

- **Price-Time Priority Matching**: Orders matched by best price first, then timestamp (FIFO)
- **Multi-Symbol Support**: Independent order books per trading symbol
- **Order Validation**: Comprehensive input validation with detailed error messages
- **High Performance**: Sub-microsecond latency for common operations
- **Type Safety**: Leverages Rust's type system to prevent runtime errors

## Performance Benchmarks

| Operation | Latency (p50) | Status |
|-----------|---------------|--------|
| Add Single Order | 420 ns | ✅ |
| Single Match | 603 ns | ✅ |
| Partial Fill (3 levels) | 1.02 µs | ✅ |
| Market Order Sweep | 2.6 µs | ✅ |
| Order Cancellation | 314 ns | ✅ |
| Cancel from Deep Book | 2.9 µs | ✅ |
| Realistic Workload | 5.6 µs | ✅ |

*Benchmarked on Apple M2 Pro, 16 GB Memory using Criterion*

## Architecture

### Core Components

#### Order
Represents a buy or sell request:
```rust
pub struct Order {
    pub id: OrderId,           // Unique identifier
    pub symbol: Symbol,        // Trading symbol (e.g., "AAPL")
    pub side: Side,           // Buy or Sell
    pub order_type: OrderType, // Market or Limit
    pub price: Option,  // Price in cents (None for market orders)
    pub quantity: Quantity,    // Number of shares
    pub timestamp: u64,        // Nanoseconds since epoch
}
```
- Orders are accompanied by verbose error messages providing detailed descriptions of why an order was rejected (if it's rejected)

#### OrderBook
Price-level sorted order storage:
- **Bids**: `BTreeMap<Price, VecDeque<Order>>` - Highest price first
- **Asks**: `BTreeMap<Price, VecDeque<Order>>` - Lowest price first
- **Order Tracking**: `HashMap<OrderId, (Side, Price)>` for O(1) cancellation

#### MatchingEngine
Processes orders and manages multiple order books:
- Maintains one order book per symbol
- Matches orders using price-time priority
- Generates trades when orders cross
- Auto-assigns order IDs and timestamps

#### Trade
Record of a matched transaction:
```rust
pub struct Trade {
    pub buyer_order_id: OrderId,
    pub seller_order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
    pub timestamp: u64,
}
```

## Building and Testing
- All of the following commands should be executed from the root directory in the terminal (rustex/)

### Build
```bash
# Development build
cargo build

# Optimized build
cargo build --release
```

### Run
```bash
cargo run
```

### Test
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_basic_buy_sell_match
```

### Benchmark
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench -- {keyword e.g cancel}
```
To view the HTML report, open target/criterion/report/index.html

### Known Bottlenecks

- Deep order books (500+ orders at same price): 47 µs
- Multi-symbol with many symbols: scales linearly

These are edge cases and don't affect typical workloads.

## Project Structure
```
rustex/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Demo application
│   ├── types/
│   │   ├── mod.rs          # Type exports
│   │   ├── order.rs        # Order and related types
│   │   └── trade.rs        # Trade type
│   ├── orderbook/
│   │   ├── mod.rs          # OrderBook exports
│   │   └── book.rs         # OrderBook implementation
│   └── matching/
│       ├── mod.rs          # MatchingEngine exports
│       └── engine.rs       # Matching logic
├── tests/
│   ├── validation_tests.rs # Order validation tests
│   └── matching_tests.rs   # Matching logic tests
├── benches/
│   └── matching_bench.rs   # Performance benchmarks
├── Cargo.toml              # Dependencies and metadata
└── README.md               # This file
```

## Future Enhancements

Potential improvements:

- Position tracking (user holdings per symbol)
- Trade history with queries
- Order modification (change price/quantity)
- Stop-loss / Stop-limit orders
- Fill-or-Kill / Immediate-or-Cancel orders
- WebSocket API for real-time updates
- Lock-free concurrent matching
- SIMD optimizations
- Persistent storage
- Market data (VWAP, OHLC, volume)