// Tests for Problem 0927: Three Equal Parts
// Java reference: src/test/java/g0901_1000/s0927_three_equal_parts/SolutionTest.java

use leetcode_in_rust::s0927::three_equal_parts::Solution;

#[test]
fn test_three_equal_parts() {
    let result = Solution::three_equal_parts(vec![1, 0, 1, 0, 1]);
    assert_eq!(result, vec![0, 3]);
}

#[test]
fn test_three_equal_parts2() {
    let result = Solution::three_equal_parts(vec![1, 1, 0, 1, 1]);
    assert_eq!(result, vec![-1, -1]);
}

#[test]
fn test_three_equal_parts3() {
    let result = Solution::three_equal_parts(vec![1, 1, 0, 0, 1]);
    assert_eq!(result, vec![0, 2]);
}
