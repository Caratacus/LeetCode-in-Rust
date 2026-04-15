// Tests for Problem 3442: Maximum Difference Between Even and Odd Frequency I
// Java reference: src/test/java/g3401_3500/s3442_maximum_difference_between_even_and_odd_frequency_i/SolutionTest.java

use leetcode_in_rust::s3442::maximum_difference_between_even_and_odd_frequency_i::Solution;

#[test]
fn test_max_difference() {
    assert_eq!(Solution::max_difference("aaaaabbc".to_string()), 3);
}

#[test]
fn test_max_difference2() {
    assert_eq!(Solution::max_difference("abcabcab".to_string()), 1);
}
