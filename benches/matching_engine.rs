use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use rust_hft_engine::{OrderBook, Side};

fn bench_limit_orders(c: &mut Criterion) {
    let mut ob = OrderBook::new();
    let price = Decimal::from(100);
    c.bench_function("limit_order_match", |b| {
        b.iter(|| {
            ob.limit_order(black_box(Side::Sell), black_box(price), black_box(10));
            ob.limit_order(black_box(Side::Buy), black_box(price), black_box(10));
        })
    });
}
criterion_group!(benches, bench_limit_orders);
criterion_main!(benches);