use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rustex::{MatchingEngine, Order, Side};

fn bench_single_order_add(c: &mut Criterion) {
    c.bench_function("add_single_limit_order", |b| {
        b.iter(|| {
            let mut engine = MatchingEngine::new();
            let order = Order::limit("AAPL".to_string(), Side::Buy, 15000, 100);
            engine.submit_order(black_box(order)).unwrap();
        });
    });
}

fn bench_single_order_cancel(c: &mut Criterion) {
    c.bench_function("cancel_single_limit_order", |b| {
        b.iter_batched(
        || {
            let mut engine = MatchingEngine::new();
            let order = Order::limit("AAPL".to_string(), Side:: Buy, 15000, 100);
            engine.submit_order(black_box(order)).unwrap();
            engine  // Return engine for benchmarking
        },
        |mut engine| {
            // Order would've been initialised with an ID of 1
            engine.cancel_order("AAPL", 1);
        },
        criterion::BatchSize::SmallInput,
    );
    });
}

fn bench_single_match(c: &mut Criterion) {
    c.bench_function("match_single_order", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                // Add sell order
                let sell = Order::limit("AAPL".to_string(), Side::Sell, 15000, 100);
                engine.submit_order(sell).unwrap();
                engine
            },
            |mut engine| {
                // Match with buy order
                let buy = Order::limit("AAPL".to_string(), Side::Buy, 15000, 100);
                engine.submit_order(black_box(buy)).unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_partial_fill(c: &mut Criterion) {
    c.bench_function("partial_fill_across_levels", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                // Add 3 sell levels
                engine.submit_order(Order::limit("AAPL".to_string(), Side::Sell, 15000, 50)).unwrap();
                engine.submit_order(Order::limit("AAPL".to_string(), Side::Sell, 15100, 75)).unwrap();
                engine.submit_order(Order::limit("AAPL".to_string(), Side::Sell, 15200, 100)).unwrap();
                engine
            },
            |mut engine| {
                // Sweep all levels
                let buy = Order::limit("AAPL".to_string(), Side::Buy, 15200, 200);
                engine.submit_order(black_box(buy)).unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_deep_book(c: &mut Criterion) {
    let mut group = c.benchmark_group("deep_book");
    
    for depth in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut engine = MatchingEngine::new();
                    // Build deep order book
                    for i in 0..depth {
                        let price = 14000 + (i * 10);
                        engine.submit_order(Order::limit("AAPL".to_string(), Side::Buy, price, 100)).unwrap();
                    }
                    engine
                },
                |mut engine| {
                    // Add order to full book
                    let order = Order::limit("AAPL".to_string(), Side::Buy, 15000, 100);
                    engine.submit_order(black_box(order)).unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_market_order(c: &mut Criterion) {
    c.bench_function("market_order_sweep", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                // Add multiple ask levels
                for i in 0..10 {
                    let price = 15000 + (i * 100);
                    engine.submit_order(Order::limit("AAPL".to_string(), Side::Sell, price, 50)).unwrap();
                }
                engine
            },
            |mut engine| {
                // Market order sweeps them all
                let order = Order::market("AAPL".to_string(), Side::Buy, 500);
                engine.submit_order(black_box(order)).unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_cancel_from_deep_book(c: &mut Criterion) {
    c.bench_function("cancel_from_deep_book", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                // Add 100 orders
                for i in 1..=100 {
                    let order = Order::limit("AAPL".to_string(), Side::Buy, 15000, 100);
                    engine.submit_order(order).unwrap();
                }
                engine
            },
            |mut engine| {
                // Cancel the 50th order (middle of book)
                engine.cancel_order("AAPL", 50);
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_multi_symbol_isolation(c: &mut Criterion) {
    c.bench_function("multi_symbol_add", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                // Pre-load with 10 symbols
                for i in 0..10 {
                    let symbol = format!("SYM{}", i);
                    for _ in 0..10 {
                        let order = Order::limit(symbol.clone(), Side::Buy, 15000, 100);
                        engine.submit_order(order).unwrap();
                    }
                }
                engine
            },
            |mut engine| {
                // Add order to 11th symbol (new symbol)
                let order = Order::limit("NEWSYM".to_string(), Side::Buy, 15000, 100);
                engine.submit_order(order).unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_realistic_workload(c: &mut Criterion) {
    c.bench_function("realistic_mixed_operations", |b| {
        b.iter(|| {
            let mut engine = MatchingEngine::new();
            
            // Add 10 orders
            for i in 1..=10 {
                let order = Order::limit("AAPL".to_string(), Side::Buy, 14900 + (i * 10), 100);
                engine.submit_order(order).unwrap();
            }
            
            // Add matching sell orders (creates 5 trades)
            for i in 1..=5 {
                let order = Order::limit("AAPL".to_string(), Side::Sell, 14900 + (i * 10), 100);
                engine.submit_order(order).unwrap();
            }
            
            // Cancel 2 orders
            engine.cancel_order("AAPL", 6);
            engine.cancel_order("AAPL", 7);
            
            // Add market order
            let market = Order::market("AAPL".to_string(), Side::Buy, 150);
            engine.submit_order(market).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_single_order_add,
    bench_single_match,
    bench_partial_fill,
    bench_deep_book,
    bench_market_order,
    bench_single_order_cancel,
    bench_cancel_from_deep_book,
    bench_multi_symbol_isolation,
    bench_realistic_workload,
);
criterion_main!(benches);