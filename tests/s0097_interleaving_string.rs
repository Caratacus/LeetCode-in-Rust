// Tests for Problem 0097: Interleaving String
// Java reference: src/test/java/g0001_0100/s0097_interleaving_string/SolutionTest.java

use leetcode_in_rust::s0097::interleaving_string::Solution;

#[test]
fn test_is_interleave() {
    assert_eq!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()), true);
}

#[test]
fn test_is_interleave2() {
    assert_eq!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()), false);
}

#[test]
fn test_is_interleave3() {
    assert_eq!(Solution::is_interleave("".to_string(), "".to_string(), "".to_string()), true);
}
