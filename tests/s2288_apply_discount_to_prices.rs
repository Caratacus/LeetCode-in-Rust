// Tests for Problem 2288: Apply Discount to Prices
// Java reference: src/test/java/g2201_2300/s2288_apply_discount_to_prices/SolutionTest.java

use leetcode_in_rust::s2288::apply_discount_to_prices::Solution;

#[test]
fn test_discount_prices() {
    assert_eq!(
        Solution::discount_prices(String::from("there are $1 $2 and 5$ candies in the shop"), 50),
        String::from("there are $0.50 $1.00 and 5$ candies in the shop")
    );
}

#[test]
fn test_discount_prices2() {
    assert_eq!(
        Solution::discount_prices(String::from("1 2 $3 4 $5 $6 7 8$ $9 $10$"), 100),
        String::from("1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$")
    );
}

#[test]
fn test_discount_prices3() {
    assert_eq!(
        Solution::discount_prices(String::from("$76111 ab $6 $"), 48),
        String::from("$39577.72 ab $3.12 $")
    );
}
