// Tests for Problem 1359: Count All Valid Pickup and Delivery Options
// Java reference: src/test/java/g1301_1400/s1359_count_all_valid_pickup_and_delivery_options/SolutionTest.java

use leetcode_in_rust::s1359::count_all_valid_pickup_and_delivery_options::Solution;

#[test]
fn test_count_orders() {
    assert_eq!(Solution::count_orders(1), 1);
}

#[test]
fn test_count_orders2() {
    assert_eq!(Solution::count_orders(2), 6);
}

#[test]
fn test_count_orders3() {
    assert_eq!(Solution::count_orders(3), 90);
}
