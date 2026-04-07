// Tests for Problem 2207: Maximize Number of Subsequences in a String
// Java reference: src/test/java/g2201_2300/s2207_maximize_number_of_subsequences_in_a_string/SolutionTest.java

use leetcode_in_rust::s2207::maximize_number_of_subsequences_in_a_string::Solution;

#[test]
fn test_maximum_subsequence_count() {
    assert_eq!(Solution::maximum_subsequence_count("abdcdbc".to_string(), "ac".to_string()), 4);
}

#[test]
fn test_maximum_subsequence_count2() {
    assert_eq!(Solution::maximum_subsequence_count("aabb".to_string(), "ab".to_string()), 6);
}

#[test]
fn test_maximum_subsequence_count3() {
    assert_eq!(Solution::maximum_subsequence_count("abdcdbc".to_string(), "aa".to_string()), 1);
}
