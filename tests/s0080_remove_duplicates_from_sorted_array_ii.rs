// Tests for Problem 0080: Remove Duplicates from Sorted Array II
// Java reference: src/test/java/g0001_0100/s0080_remove_duplicates_from_sorted_array_ii/SolutionTest.java

use leetcode_in_rust::s0080::remove_duplicates_from_sorted_array_ii::Solution;

#[test]
fn test_remove_duplicates() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let len = Solution::remove_duplicates(&mut nums) as usize;
    assert_eq!(&nums[..len], &[1, 1, 2, 2, 3]);
}

#[test]
fn test_remove_duplicates2() {
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let len = Solution::remove_duplicates(&mut nums) as usize;
    assert_eq!(&nums[..len], &[0, 0, 1, 1, 2, 3, 3]);
}
