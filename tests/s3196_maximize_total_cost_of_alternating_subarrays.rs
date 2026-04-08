// Tests for Problem 3196: Maximize Total Cost of Alternating Subarrays
// Java reference: src/test/java/g3101_3200/s3196_maximize_total_cost_of_alternating_subarrays/SolutionTest.java

use leetcode_in_rust::s3196::maximize_total_cost_of_alternating_subarrays::Solution;

#[test]
fn test_maximum_total_cost() {
    assert_eq!(Solution::maximum_total_cost(vec![1, -2, 3, 4]), 10);
}

#[test]
fn test_maximum_total_cost2() {
    assert_eq!(Solution::maximum_total_cost(vec![1, -1, 1, -1]), 4);
}

#[test]
fn test_maximum_total_cost3() {
    assert_eq!(Solution::maximum_total_cost(vec![0]), 0);
}

#[test]
fn test_maximum_total_cost4() {
    assert_eq!(Solution::maximum_total_cost(vec![1, -1]), 2);
}
