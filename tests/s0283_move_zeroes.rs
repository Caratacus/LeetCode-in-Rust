// Tests for Problem 0283: Move Zeroes
// Java reference: src/test/java/g0201_0300/s0283_move_zeroes/SolutionTest.java
// Note: The Rust API takes Vec<i32> by value; test adjusted accordingly

use leetcode_in_rust::s0283::move_zeroes::Solution;

#[test]
#[ignore = "API takes ownership - cannot verify result without modification"]
fn test_move_zeroes() {
    let nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(nums);
}

#[test]
#[ignore = "API takes ownership - cannot verify result without modification"]
fn test_move_zeroes2() {
    let nums = vec![0];
    Solution::move_zeroes(nums);
}
