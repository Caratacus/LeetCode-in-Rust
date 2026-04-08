// Tests for Problem 3014: Minimum Number of Pushes to Type Word I
// Java reference: src/test/java/g3001_3100/s3014_minimum_number_of_pushes_to_type_word_i/SolutionTest.java

use leetcode_in_rust::s3014::minimum_number_of_pushes_to_type_word_i::Solution;

#[test]
fn test_minimum_pushes() {
    assert_eq!(Solution::minimum_pushes(String::from("abcde")), 5);
}

#[test]
fn test_minimum_pushes2() {
    assert_eq!(Solution::minimum_pushes(String::from("xycdefghij")), 12);
}
