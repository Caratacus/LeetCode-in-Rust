// Tests for Problem 3013: Divide an Array Into Subarrays With Minimum Cost II
// Java reference: src/test/java/g3001_3100/s3013_divide_an_array_into_subarrays_with_minimum_cost_ii/SolutionTest.java

use leetcode_in_rust::s3013::divide_an_array_into_subarrays_with_minimum_cost_ii::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
}

#[test]
fn test_minimum_cost3() {
    assert_eq!(Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);
}
