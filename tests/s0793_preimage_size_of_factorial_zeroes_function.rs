// Tests for Problem 0793: Preimage Size of Factorial Zeroes Function
// Java reference: src/test/java/g0701_0800/s0793_preimage_size_of_factorial_zeroes_function/SolutionTest.java

use leetcode_in_rust::s0793::preimage_size_of_factorial_zeroes_function::Solution;

#[test]
fn test_preimage_size_fzf() {
    assert_eq!(Solution::preimage_size_fzf(0), 5);
}

#[test]
fn test_preimage_size_fzf2() {
    assert_eq!(Solution::preimage_size_fzf(5), 0);
}

#[test]
fn test_preimage_size_fzf3() {
    assert_eq!(Solution::preimage_size_fzf(3), 5);
}
