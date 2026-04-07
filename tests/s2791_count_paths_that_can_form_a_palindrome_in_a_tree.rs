// Tests for Problem 2791: Count Paths That Can Form a Palindrome in a Tree
// Java reference: src/test/java/g2701_2800/s2791_count_paths_that_can_form_a_palindrome_in_a_tree/SolutionTest.java

use leetcode_in_rust::s2791::count_paths_that_can_form_a_palindrome_in_a_tree::Solution;

#[test]
fn test_count_palindrome_paths() {
    assert_eq!(Solution::count_palindrome_paths(vec![-1, 0, 0, 1, 1, 2], "acaabc".to_string()), 8);
}

#[test]
fn test_count_palindrome_paths2() {
    assert_eq!(Solution::count_palindrome_paths(vec![-1, 0, 0, 0, 0], "aaaaa".to_string()), 10);
}
