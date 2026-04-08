// Tests for Problem 3139: Minimum Cost to Equalize Array
// Java reference: src/test/java/g3101_3200/s3139_minimum_cost_to_equalize_array/SolutionTest.java

use leetcode_in_rust::s3139::minimum_cost_to_equalize_array::Solution;
#[test]
fn test_min_cost_to_equalize_array() {
    assert_eq!(Solution::min_cost_to_equalize_array(vec![4, 1], 5, 2), 15);
}
#[test]
fn test_min_cost_to_equalize_array2() {
    assert_eq!(Solution::min_cost_to_equalize_array(vec![2, 3, 3, 3, 5], 2, 1), 6);
}
#[test]
fn test_min_cost_to_equalize_array3() {
    assert_eq!(Solution::min_cost_to_equalize_array(vec![3, 5, 3], 1, 3), 4);
}
