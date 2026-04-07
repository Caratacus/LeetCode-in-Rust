// Tests for Problem 1987: Number of Unique Good Subsequences
// Java reference: src/test/java/g1901_2000/s1987_number_of_unique_good_subsequences/SolutionTest.java

use leetcode_in_rust::s1987::number_of_unique_good_subsequences::Solution;

#[test]
fn test_number_of_unique_good_subsequences() {
    assert_eq!(Solution::number_of_unique_good_subsequences("001".to_string()), 2);
}

#[test]
fn test_number_of_unique_good_subsequences2() {
    assert_eq!(Solution::number_of_unique_good_subsequences("11".to_string()), 2);
}

#[test]
fn test_number_of_unique_good_subsequences3() {
    assert_eq!(Solution::number_of_unique_good_subsequences("101".to_string()), 5);
}
