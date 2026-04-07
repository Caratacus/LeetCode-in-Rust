// Tests for Problem 2560: House Robber IV
// Java reference: src/test/java/g2501_2600/s2560_house_robber_iv/SolutionTest.java

use leetcode_in_rust::s2560::house_robber_iv::Solution;

#[test]
fn test_min_capability() {
    assert_eq!(Solution::min_capability(vec![2, 3, 5, 9], 2), 5);
}

#[test]
fn test_min_capability2() {
    assert_eq!(Solution::min_capability(vec![2, 7, 9, 3, 1], 2), 2);
}
