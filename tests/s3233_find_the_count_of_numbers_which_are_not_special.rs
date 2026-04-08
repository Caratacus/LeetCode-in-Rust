// Tests for Problem 3233: Find the Count of Numbers Which Are Not Special
// Java reference: src/test/java/g3201_3300/s3233_find_the_count_of_numbers_which_are_not_special/SolutionTest.java

use leetcode_in_rust::s3233::find_the_count_of_numbers_which_are_not_special::Solution;

#[test]
fn test_non_special_count() {
    assert_eq!(Solution::non_special_count(5, 7), 3);
}

#[test]
fn test_non_special_count2() {
    assert_eq!(Solution::non_special_count(4, 16), 11);
}
