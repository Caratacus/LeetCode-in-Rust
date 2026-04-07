// Tests for Problem 2558: Take Gifts From the Richest Pile
// Java reference: src/test/java/g2501_2600/s2558_take_gifts_from_the_richest_pile/SolutionTest.java

use leetcode_in_rust::s2558::take_gifts_from_the_richest_pile::Solution;

#[test]
fn test_pick_gifts() {
    assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
}

#[test]
fn test_pick_gifts2() {
    assert_eq!(Solution::pick_gifts(vec![1, 1, 1, 1], 4), 4);
}
