// Tests for Problem 2561: Rearranging Fruits
// Java reference: src/test/java/g2501_2600/s2561_rearranging_fruits/SolutionTest.java

use leetcode_in_rust::s2561::rearranging_fruits::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]), 1);
}

#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(vec![2, 3, 4, 1], vec![3, 2, 5, 1]), -1);
}
