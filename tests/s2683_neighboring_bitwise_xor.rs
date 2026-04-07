// Tests for Problem 2683: Neighboring Bitwise XOR
// Java reference: src/test/java/g2601_2700/s2683_neighboring_bitwise_xor/SolutionTest.java

use leetcode_in_rust::s2683::neighboring_bitwise_xor::Solution;

#[test]
fn test_does_valid_array_exist() {
    assert_eq!(Solution::does_valid_array_exist(vec![1, 1, 0]), true);
}

#[test]
fn test_does_valid_array_exist2() {
    assert_eq!(Solution::does_valid_array_exist(vec![1, 1]), true);
}

#[test]
fn test_does_valid_array_exist3() {
    assert_eq!(Solution::does_valid_array_exist(vec![1, 0]), false);
}
