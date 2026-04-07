// Tests for Problem 2547: Minimum Cost to Split an Array
// Java reference: src/test/java/g2501_2600/s2547_minimum_cost_to_split_an_array/SolutionTest.java
use leetcode_in_rust::s2547::minimum_cost_to_split_an_array::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1, 3, 3], 2), 8);
}
#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 2), 6);
}
#[test]
fn test_min_cost3() {
    assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 5), 10);
}
