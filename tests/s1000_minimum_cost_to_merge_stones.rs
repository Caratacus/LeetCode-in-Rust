// Tests for Problem 1000: Minimum Cost to Merge Stones
// Java reference: src/test/java/g0901_1000/s1000_minimum_cost_to_merge_stones/SolutionTest.java

use leetcode_in_rust::s1000::minimum_cost_to_merge_stones::Solution;

#[test]
fn test_merge_stones() {
    let result = Solution::merge_stones(vec![3, 2, 4, 1], 2);
    assert_eq!(result, 20);
}

#[test]
fn test_merge_stones2() {
    let result = Solution::merge_stones(vec![3, 2, 4, 1], 3);
    assert_eq!(result, -1);
}

#[test]
fn test_merge_stones3() {
    let result = Solution::merge_stones(vec![3, 5, 1, 2, 6], 3);
    assert_eq!(result, 25);
}
