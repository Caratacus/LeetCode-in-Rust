// Tests for Problem 2144: Minimum Cost of Buying Candies With Discount
// Java reference: src/test/java/g2101_2200/s2144_minimum_cost_of_buying_candies_with_discount/SolutionTest.java

use leetcode_in_rust::s2144::minimum_cost_of_buying_candies_with_discount::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(Solution::minimum_cost(vec![1, 2, 3]), 5);
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
}

#[test]
fn test_minimum_cost3() {
    assert_eq!(Solution::minimum_cost(vec![5, 5]), 10);
}
