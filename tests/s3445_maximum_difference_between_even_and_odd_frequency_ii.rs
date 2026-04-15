// Tests for Problem 3445: Maximum Difference Between Even and Odd Frequency II
// Java reference: src/test/java/g3401_3500/s3445_maximum_difference_between_even_and_odd_frequency_ii/SolutionTest.java

use leetcode_in_rust::s3445::maximum_difference_between_even_and_odd_frequency_ii::Solution;

#[test]
fn test_max_difference() {
    assert_eq!(Solution::max_difference("12233".to_string(), 4), -1);
}

#[test]
fn test_max_difference2() {
    assert_eq!(Solution::max_difference("1122211".to_string(), 3), 1);
}

#[test]
fn test_max_difference3() {
    assert_eq!(Solution::max_difference("110".to_string(), 3), -1);
}
