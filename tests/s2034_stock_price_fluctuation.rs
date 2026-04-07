// Tests for Problem 2034: Stock Price Fluctuation
// Java reference: src/test/java/g2001_2100/s2034_stock_price_fluctuation/StockPriceTest.java

use leetcode_in_rust::s2034::stock_price_fluctuation::StockPrice;

#[test]
fn test_stock_price_test() {
    let mut stock_price = StockPrice::new();
    stock_price.update(1, 10);
    stock_price.update(2, 5);
    assert_eq!(stock_price.current(), 5);
    assert_eq!(stock_price.maximum(), 10);
    stock_price.update(1, 3);
    assert_eq!(stock_price.maximum(), 5);
    stock_price.update(4, 2);
    assert_eq!(stock_price.minimum(), 2);
}
