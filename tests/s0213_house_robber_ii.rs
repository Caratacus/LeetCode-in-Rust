// Tests for Problem 0213: House Robber II
// Java reference: src/test/java/g0201_0300/s0213_house_robber_ii/SolutionTest.java

use leetcode_in_rust::s0213::house_robber_ii::Solution;

#[test]
fn test_rob() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
}

#[test]
fn test_rob2() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
fn test_rob3() {
    assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
}

#[test]
fn test_rob4() {
    assert_eq!(Solution::rob(vec![]), 0);
    assert_eq!(Solution::rob(vec![1]), 1);
    assert_eq!(Solution::rob(vec![1, 2]), 2);
}
