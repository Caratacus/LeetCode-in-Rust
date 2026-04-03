// Tests for Problem 1312: Minimum Insertion Steps to Make a String Palindrome
// Java reference: src/test/java/g1301_1400/s1312_minimum_insertion_steps_to_make_a_string_palindrome/SolutionTest.java

use leetcode_in_rust::s1312::minimum_insertion_steps_to_make_a_string_palindrome::Solution;

#[test]
fn test_min_insertions() {
    assert_eq!(Solution::min_insertions("zzazz".to_string()), 0);
}

#[test]
fn test_min_insertions2() {
    assert_eq!(Solution::min_insertions("mbadm".to_string()), 2);
}

#[test]
fn test_min_insertions3() {
    assert_eq!(Solution::min_insertions("leetcode".to_string()), 5);
}
