use rust_hft_engine::{OrderBook, Side};
use rust_decimal::Decimal;

fn main() {
    let mut ob = OrderBook::new();
    let p = Decimal::from;
    println!("--- Engine Initialized ---");
    ob.limit_order(Side::Sell, p(100), 10);
    ob.limit_order(Side::Buy, p(110), 12);
    println!("Matching complete. Ready for high-frequency load.");
}