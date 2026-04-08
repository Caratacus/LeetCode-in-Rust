// Tests for Problem 3146: Permutation Difference Between Two Strings
// Java reference: src/test/java/g3101_3200/s3146_permutation_difference_between_two_strings/SolutionTest.java

use leetcode_in_rust::s3146::permutation_difference_between_two_strings::Solution;
#[test]
fn test_find_permutation_difference() {
    assert_eq!(Solution::find_permutation_difference(String::from("abc"), String::from("bac")), 2);
}
#[test]
fn test_find_permutation_difference2() {
    assert_eq!(Solution::find_permutation_difference(String::from("abcde"), String::from("edbac"), 12);
}
