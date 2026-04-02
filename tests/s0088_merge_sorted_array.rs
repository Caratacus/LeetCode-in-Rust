// Tests for Problem 0088: Merge Sorted Array
// Java reference: src/test/java/g0001_0100/s0088_merge_sorted_array/SolutionTest.java

use leetcode_in_rust::s0088::merge_sorted_array::Solution;

#[test]
fn test_merge() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}

#[test]
fn test_merge2() {
    let mut nums1 = vec![1];
    let nums2: Vec<i32> = vec![];
    Solution::merge(&mut nums1, 1, nums2, 0);
    assert_eq!(nums1, vec![1]);
}

#[test]
fn test_merge3() {
    let mut nums1 = vec![0];
    let nums2 = vec![1];
    Solution::merge(&mut nums1, 0, nums2, 1);
    assert_eq!(nums1, vec![1]);
}
