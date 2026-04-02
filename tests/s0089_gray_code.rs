// Tests for Problem 0089: Gray Code
// Java reference: src/test/java/g0001_0100/s0089_gray_code/SolutionTest.java

use leetcode_in_rust::s0089::gray_code::Solution;

#[test]
fn test_gray_code() {
    let result = Solution::gray_code(2);
    assert_eq!(result, vec![0, 1, 3, 2]);
}

#[test]
fn test_gray_code2() {
    let result = Solution::gray_code(1);
    assert_eq!(result, vec![0, 1]);
}
