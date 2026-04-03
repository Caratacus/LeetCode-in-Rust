// Tests for Problem 1643: Kth Smallest Instructions
// Java reference: src/test/java/g1601_1700/s1643_kth_smallest_instructions/SolutionTest.java

use leetcode_in_rust::s1643::kth_smallest_instructions::Solution;

#[test]
fn test_kth_smallest_path() {
    assert_eq!(Solution::kth_smallest_path(vec![2, 3], 1), "HHHVV");
}

#[test]
fn test_kth_smallest_path2() {
    assert_eq!(Solution::kth_smallest_path(vec![2, 3], 2), "HHVHV");
}

#[test]
fn test_kth_smallest_path3() {
    assert_eq!(Solution::kth_smallest_path(vec![2, 3], 3), "HHVVH");
}
