// Tests for Problem 2260: Minimum Consecutive Cards to Pick Up
// Java reference: src/test/java/g2201_2300/s2260_minimum_consecutive_cards_to_pick_up/SolutionTest.java

use leetcode_in_rust::s2260::minimum_consecutive_cards_to_pick_up::Solution;

#[test]
fn test_minimum_card_pickup() {
    assert_eq!(Solution::minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]), 4);
}

#[test]
fn test_minimum_card_pickup2() {
    assert_eq!(Solution::minimum_card_pickup(vec![1, 0, 5, 3]), -1);
}
