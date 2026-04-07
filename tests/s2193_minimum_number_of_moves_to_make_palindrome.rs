// Tests for Problem 2193: Minimum Number of Moves to Make Palindrome
// Java reference: src/test/java/g2101_2200/s2193_minimum_number_of_moves_to_make_palindrome/SolutionTest.java

use leetcode_in_rust::s2193::minimum_number_of_moves_to_make_palindrome::Solution;

#[test]
fn test_min_moves_to_make_palindrome() {
    assert_eq!(Solution::min_moves_to_make_palindrome("aabb".to_string()), 2);
}

#[test]
fn test_min_moves_to_make_palindrome2() {
    assert_eq!(Solution::min_moves_to_make_palindrome("letelt".to_string()), 2);
}

#[test]
fn test_min_moves_to_make_palindrome3() {
    assert_eq!(
        Solution::min_moves_to_make_palindrome("skwhhaaunskegmdtutlgtteunmuuludii".to_string()),
        163
    );
}
