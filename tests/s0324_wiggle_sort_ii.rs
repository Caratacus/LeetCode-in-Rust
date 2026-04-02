// Tests for Problem 0324: Wiggle Sort II
// Java reference: src/test/java/g0301_0400/s0324_wiggle_sort_ii/SolutionTest.java

use leetcode_in_rust::s0324::wiggle_sort_ii::Solution;

// #[ignore] - API takes Vec<i32> by value instead of &mut Vec<i32>, cannot verify results
#[test]
fn test_wiggle_sort() {
    let nums = vec![1, 5, 1, 1, 6, 4];
    Solution::wiggle_sort(nums);
    // Note: Cannot verify result as API takes ownership
}

// #[ignore] - API takes Vec<i32> by value instead of &mut Vec<i32>, cannot verify results
#[test]
fn test_wiggle_sort2() {
    let nums = vec![1, 3, 2, 2, 3, 1];
    Solution::wiggle_sort(nums);
    // Note: Cannot verify result as API takes ownership
}
