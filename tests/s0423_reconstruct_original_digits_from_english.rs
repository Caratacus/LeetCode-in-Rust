// Tests for Problem 0423: Reconstruct Original Digits from English
// Java reference: src/test/java/g0401_0500/s0423_reconstruct_original_digits_from_english/SolutionTest.java

use leetcode_in_rust::s0423::reconstruct_original_digits_from_english::Solution;

#[test]
fn test_original_digits() {
    assert_eq!(Solution::original_digits("owoztneoer".to_string()), "012");
}

#[test]
fn test_original_digits2() {
    assert_eq!(Solution::original_digits("fviefuro".to_string()), "45");
}
