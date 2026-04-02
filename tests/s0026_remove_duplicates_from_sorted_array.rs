// Tests for Problem 0026: Remove Duplicates from Sorted Array
// Java reference: src/test/java/g0001_0100/s0026_remove_duplicates_from_sorted_array/SolutionTest.java

use leetcode_in_rust::s0026::remove_duplicates_from_sorted_array::Solution;

#[test]
fn test_remove_duplicates() {
    let mut nums = vec![1, 1, 2];
    let len = Solution::remove_duplicates(&mut nums) as usize;
    assert_eq!(&nums[..len], &[1, 2]);
}

#[test]
fn test_remove_duplicates2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let len = Solution::remove_duplicates(&mut nums) as usize;
    assert_eq!(&nums[..len], &[0, 1, 2, 3, 4]);
}
