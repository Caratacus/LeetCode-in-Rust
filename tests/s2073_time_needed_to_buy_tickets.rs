// Tests for Problem 2073: Time Needed to Buy Tickets
// Java reference: src/test/java/g2001_2100/s2073_time_needed_to_buy_tickets/SolutionTest.java

use leetcode_in_rust::s2073::time_needed_to_buy_tickets::Solution;

#[test]
fn test_time_required_to_buy() {
    assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
}

#[test]
fn test_time_required_to_buy2() {
    assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
}
