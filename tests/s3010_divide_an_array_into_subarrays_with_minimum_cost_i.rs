// Tests for Problem 3010: Divide an Array Into Subarrays With Minimum Cost I
// Java reference: src/test/java/g3001_3100/s3010_divide_an_array_into_subarrays_with_minimum_cost_i/SolutionTest.java

use leetcode_in_rust::s3010::divide_an_array_into_subarrays_with_minimum_cost_i::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 12]), 6);
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(Solution::minimum_cost(vec![5, 4, 3]), 12);
}

#[test]
fn test_minimum_cost3() {
    assert_eq!(Solution::minimum_cost(vec![10, 3, 1, 1]), 12);
}
