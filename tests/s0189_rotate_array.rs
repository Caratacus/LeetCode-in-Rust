// Tests for Problem 0189: Rotate Array
// Java reference: src/test/java/g0181_0200/s0189_rotate_array/SolutionTest.java

use leetcode_in_rust::s0189::rotate_array::Solution;

#[test]
fn test_rotate() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(nums, 3);
    // Function takes ownership, modifies in place
}

#[test]
fn test_rotate2() {
    let nums = vec![-1, -100, 3, 99];
    Solution::rotate(nums, 2);
}
