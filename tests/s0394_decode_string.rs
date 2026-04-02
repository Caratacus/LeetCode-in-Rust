// Tests for Problem 0394: Decode String
// Java reference: src/test/java/g0301_0400/s0394_decode_string/SolutionTest.java

use leetcode_in_rust::s0394::decode_string::Solution;

#[test]
fn test_decode_string() {
    assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc");
}

#[test]
fn test_decode_string2() {
    assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc");
}

#[test]
fn test_decode_string3() {
    assert_eq!(Solution::decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef");
}

#[test]
fn test_decode_string4() {
    assert_eq!(Solution::decode_string("abc3[cd]xyz".to_string()), "abccdcdcdxyz");
}
