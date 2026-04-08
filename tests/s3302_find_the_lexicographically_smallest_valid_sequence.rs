// Tests for Problem 3302: Find the Lexicographically Smallest Valid Sequence
// Java reference: src/test/java/g3301_3400/s3302_find_the_lexicographically_smallest_valid_sequence/SolutionTest.java

use leetcode_in_rust::s3302::find_the_lexicographically_smallest_valid_sequence::Solution;

#[test]
fn test_valid_sequence() {
    assert_eq!(Solution::valid_sequence("vbcca".to_string(), "abc".to_string()), vec![0, 1, 2]);
}

#[test]
fn test_valid_sequence2() {
    assert_eq!(Solution::valid_sequence("bacdc".to_string(), "abc".to_string()), vec![1, 2, 4]);
}

#[test]
fn test_valid_sequence3() {
    assert_eq!(Solution::valid_sequence("aaaaaa".to_string(), "aaabc".to_string()), vec![]);
}

#[test]
fn test_valid_sequence4() {
    assert_eq!(Solution::valid_sequence("abc".to_string(), "ab".to_string()), vec![0, 1]);
}
