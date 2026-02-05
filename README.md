# Rustex - Low-Latency Order Matching Engine

A high-performance order book and matching engine built in Rust, designed for microsecond-level latency.

## Features (Planned)
- [ ] Order book with price-time priority
- [ ] Market and limit order support
- [ ] Real-time order matching
- [ ] WebSocket API for order submission
- [ ] Performance benchmarks (<10μs order processing)

## Tech Stack
- Rust (async/tokio)
- WebSockets for real-time data
- Custom data structures for order book

## Performance Goals
- Order processing: <10 microseconds (p99)
- Throughput: 100k+ orders/second
- Memory efficient order book

## Development
```bash
cargo build
cargo test
cargo bench
```

## Status
Preparing the project and organising structure