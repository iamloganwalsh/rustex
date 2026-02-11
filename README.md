# Rustex - Low-Latency Order Matching Engine

A high-performance order matching engine built in Rust, implementing price-time priority matching for limit and market orders.

## Features

- **Price-Time Priority**: Orders matched by best price first, then by timestamp
- **Order Types**: Support for limit and market orders
- **Order Management**: O(1) order cancellation using HashMap indexing
- **Partial Fills**: Orders can match across multiple price levels
- **Order Book Visualization**: Real-time depth display

## Architecture

- **OrderBook**: BTreeMap-based structure for sorted price levels with FIFO queues
- **MatchingEngine**: Core matching logic for buy/sell order processing
- **Trade Generation**: Automatic trade creation when orders match

## Usage (Syntax)
```rust
let mut engine = MatchingEngine::new();

// Add limit order
engine.process_order(Order::new_limit(unique_order_id, Side::Buy, price, quantity, timestamp));

// Add market order
engine.process_order(Order::new_market(unique_order_id, Side::Sell, quantity, timestamp));  // No price because it's a market order

// Display order book
engine.order_book.display();

// Cancel order
engine.order_book.cancel_order(order_id_to_cancel);
```

## Running
```bash
cargo run
```

## Performance Goals

- Order processing: <10 microseconds (p99) [planned optimization]
- Throughput: 100k+ orders/second [planned optimization]

## Tech Stack

- Rust 2021
- BTreeMap for price-level sorting
- HashMap for O(1) order lookup
- VecDeque for FIFO queues at each price level

## Roadmap

- [x] Core matching engine
- [x] Limit and market orders
- [x] Order cancellation
- [x] Order book visualization
- [ ] Performance benchmarking
- [ ] Lock-free order book
- [ ] SIMD optimization