// Tests for Problem 1475: Final Prices With a Special Discount in a Shop
// Java reference: src/test/java/g1401_1500/s1475_final_prices_with_a_special_discount_in_a_shop/SolutionTest.java

use leetcode_in_rust::s1475::final_prices_with_a_special_discount_in_a_shop::Solution;

#[test]
fn test_final_prices() {
    assert_eq!(Solution::final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
}

#[test]
fn test_final_prices2() {
    assert_eq!(Solution::final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_final_prices3() {
    assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
}
