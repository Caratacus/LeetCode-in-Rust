// Tests for Problem 0198: House Robber
// Java reference: src/test/java/g0181_0200/s0198_house_robber/SolutionTest.java

use leetcode_in_rust::s0198::house_robber::Solution;

#[test]
fn test_rob() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
fn test_rob2() {
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}

#[test]
fn test_rob3() {
    assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
}
