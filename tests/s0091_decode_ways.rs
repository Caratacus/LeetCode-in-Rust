// Tests for Problem 0091: Decode Ways
// Java reference: src/test/java/g0001_0100/s0091_decode_ways/SolutionTest.java

use leetcode_in_rust::s0091::decode_ways::Solution;

#[test]
fn test_num_decodings() {
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
}

#[test]
fn test_num_decodings2() {
    assert_eq!(Solution::num_decodings("226".to_string()), 3);
}

#[test]
fn test_num_decodings3() {
    assert_eq!(Solution::num_decodings("06".to_string()), 0);
}
