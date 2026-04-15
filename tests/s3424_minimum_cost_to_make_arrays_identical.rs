// Tests for Problem 3424: Minimum Cost to Make Arrays Identical
// Java reference: src/test/java/g3401_3500/s3424_minimum_cost_to_make_arrays_identical/SolutionTest.java

use leetcode_in_rust::s3424::minimum_cost_to_make_arrays_identical::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(vec![-7, 9, 5], vec![7, -2, -5], 2), 13i64);
}

#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(vec![2, 1], vec![2, 1], 0), 0i64);
}
