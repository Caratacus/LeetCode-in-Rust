// Tests for Problem 0632: Smallest Range Covering Elements from K Lists
// Java reference: src/test/java/g0601_0700/s0632_smallest_range_covering_elements_from_k_lists/SolutionTest.java

use leetcode_in_rust::s0632::smallest_range_covering_elements_from_k_lists::Solution;

#[test]
fn test_smallest_range() {
    let nums = vec![
        vec![4, 10, 15, 24, 26],
        vec![0, 9, 12, 20],
        vec![5, 18, 22, 30],
    ];
    let result = Solution::smallest_range(nums);
    assert_eq!(result, vec![20, 24]);
}

#[test]
fn test_smallest_range2() {
    let nums = vec![
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![1, 2, 3],
    ];
    let result = Solution::smallest_range(nums);
    assert_eq!(result, vec![1, 1]);
}
