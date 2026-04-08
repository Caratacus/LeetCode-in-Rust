// Tests for Problem 3234: Count the Number of Substrings With Dominant Ones
// Java reference: src/test/java/g3201_3300/s3234_count_the_number_of_substrings_with_dominant_ones/SolutionTest.java

use leetcode_in_rust::s3234::count_the_number_of_substrings_with_dominant_ones::Solution;

#[test]
fn test_number_of_substrings() {
    assert_eq!(Solution::number_of_substrings("00011".to_string()), 5);
}

#[test]
fn test_number_of_substrings2() {
    assert_eq!(Solution::number_of_substrings("101101".to_string()), 16);
}
