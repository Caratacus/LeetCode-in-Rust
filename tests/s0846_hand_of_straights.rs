// Tests for Problem 0846: Hand of Straights
// Java reference: src/test/java/g0801_0900/s0846_hand_of_straights/SolutionTest.java

use leetcode_in_rust::s0846::hand_of_straights::Solution;

#[test]
fn test_is_n_straight_hand() {
    assert_eq!(
        Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3),
        true
    );
}

#[test]
fn test_is_n_straight_hand2() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
}
