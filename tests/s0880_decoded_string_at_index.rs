// Tests for Problem 0880: Decoded String at Index
// Java reference: src/test/java/g0801_0900/s0880_decoded_string_at_index/SolutionTest.java

use leetcode_in_rust::s0880::decoded_string_at_index::Solution;

#[test]
fn test_decode_at_index() {
    assert_eq!(Solution::decode_at_index("leet2code3".to_string(), 10), "o");
}

#[test]
fn test_decode_at_index2() {
    assert_eq!(Solution::decode_at_index("ha22".to_string(), 5), "h");
}

#[test]
fn test_decode_at_index3() {
    assert_eq!(Solution::decode_at_index("a2345678999999999999999".to_string(), 1), "a");
}
