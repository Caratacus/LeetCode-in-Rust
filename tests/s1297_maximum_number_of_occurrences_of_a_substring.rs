// Tests for Problem 1297: Maximum Number of Occurrences of a Substring
// Java reference: src/test/java/g1201_1300/s1297_maximum_number_of_occurrences_of_a_substring/SolutionTest.java

use leetcode_in_rust::s1297::maximum_number_of_occurrences_of_a_substring::Solution;

#[test]
fn test_max_freq() {
    assert_eq!(Solution::max_freq("aababcaab".to_string(), 2, 3, 4), 2);
}

#[test]
fn test_max_freq2() {
    assert_eq!(Solution::max_freq("aaaa".to_string(), 1, 3, 3), 2);
}
