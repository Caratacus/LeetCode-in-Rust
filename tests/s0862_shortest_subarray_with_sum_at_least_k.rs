// Tests for Problem 0862: Shortest Subarray with Sum at Least K
// Java reference: src/test/java/g0801_0900/s0862_shortest_subarray_with_sum_at_least_k/SolutionTest.java

use leetcode_in_rust::s0862::shortest_subarray_with_sum_at_least_k::Solution;

#[test]
fn test_shortest_subarray() {
    assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
}

#[test]
fn test_shortest_subarray2() {
    assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1);
}

#[test]
fn test_shortest_subarray3() {
    assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3);
}
