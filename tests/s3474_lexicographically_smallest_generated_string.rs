// Tests for Problem 3474: Lexicographically Smallest Generated String
// Java reference: src/test/java/g3401_3500/s3474_lexicographically_smallest_generated_string/SolutionTest.java

use leetcode_in_rust::s3474::lexicographically_smallest_generated_string::Solution;

#[test]
fn test_generate_string() {
    assert_eq!(Solution::generate_string("TFTF".to_string(), "ab".to_string()), "ababa".to_string());
}

#[test]
fn test_generate_string2() {
    assert_eq!(Solution::generate_string("TFTF".to_string(), "abc".to_string()), "".to_string());
}

#[test]
fn test_generate_string3() {
    assert_eq!(Solution::generate_string("F".to_string(), "d".to_string()), "a".to_string());
}

#[test]
fn test_generate_string4() {
    assert_eq!(Solution::generate_string("TTFFT".to_string(), "fff".to_string()), "".to_string());
}

#[test]
fn test_generate_string5() {
    assert_eq!(Solution::generate_string("FFTFFF".to_string(), "a".to_string()), "bbabbb".to_string());
}
