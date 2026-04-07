// Tests for Problem 2842: Count K Subsequences of a String With Maximum Beauty
// Java reference: src/test/java/g2801_2900/s2842_count_k_subsequences_of_a_string_with_maximum_beauty/SolutionTest.java

use leetcode_in_rust::s2842::count_k_subsequences_of_a_string_with_maximum_beauty::Solution;

#[test]
fn test_count_k_subsequences_with_max_beauty() {
    assert_eq!(Solution::count_k_subsequences_with_max_beauty("bcca".to_string(), 2), 4);
}

#[test]
fn test_count_k_subsequences_with_max_beauty2() {
    assert_eq!(Solution::count_k_subsequences_with_max_beauty("abbcd".to_string(), 4), 2);
}
