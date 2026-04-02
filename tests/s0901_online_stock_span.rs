// Tests for Problem 0901: Online Stock Span
// Java reference: src/test/java/g0901_1000/s0901_online_stock_span/SolutionTest.java

use leetcode_in_rust::s0901::online_stock_span::StockSpanner;

#[test]
fn test_stock_spanner() {
    let mut stock_spanner = StockSpanner::new();
    assert_eq!(stock_spanner.next(100), 1);
    assert_eq!(stock_spanner.next(80), 1);
    assert_eq!(stock_spanner.next(60), 1);
    assert_eq!(stock_spanner.next(70), 2);
    assert_eq!(stock_spanner.next(60), 1);
    assert_eq!(stock_spanner.next(75), 4);
    assert_eq!(stock_spanner.next(85), 6);
}
