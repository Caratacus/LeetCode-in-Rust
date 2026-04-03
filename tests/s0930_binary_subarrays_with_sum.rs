// Tests for Problem 0930: Binary Subarrays With Sum
// Java reference: src/test/java/g0901_1000/s0930_binary_subarrays_with_sum/SolutionTest.java

use leetcode_in_rust::s0930::binary_subarrays_with_sum::Solution;

#[test]
fn test_num_subarrays_with_sum() {
    let result = Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2);
    assert_eq!(result, 4);
}

#[test]
fn test_num_subarrays_with_sum2() {
    let result = Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0);
    assert_eq!(result, 15);
}
