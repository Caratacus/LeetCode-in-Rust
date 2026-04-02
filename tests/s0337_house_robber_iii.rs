// Tests for Problem 0337: House Robber III
// Java reference: src/test/java/g0301_0400/s0337_house_robber_iii/SolutionTest.java

use leetcode_in_rust::s0337::house_robber_iii::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_rob() {
    let root = tree_from_vec(vec![Some(3), Some(2), Some(3), None, Some(3), None, Some(1)]);
    assert_eq!(Solution::rob(root), 7);
}

#[test]
fn test_rob2() {
    let root = tree_from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1)]);
    assert_eq!(Solution::rob(root), 9);
}
