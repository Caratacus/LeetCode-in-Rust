// Tests for Problem 0643: Maximum Average Subarray I
// Java reference: src/test/java/g0601_0700/s0643_maximum_average_subarray_i/SolutionTest.java

use leetcode_in_rust::s0643::maximum_average_subarray_i::Solution;

#[test]
fn test_find_max_average() {
    let result = Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4);
    assert!((result - 12.75).abs() < 0.00001);
}

#[test]
fn test_find_max_average2() {
    let result = Solution::find_max_average(vec![5], 1);
    assert!((result - 5.0).abs() < 0.00001);
}
