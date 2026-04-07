// Tests for Problem 2267: Check if There Is a Valid Parentheses String Path
// Java reference: src/test/java/g2201_2300/s2267_check_if_there_is_a_valid_parentheses_string_path/SolutionTest.java

use leetcode_in_rust::s2267::check_if_there_is_a_valid_parentheses_string_path::Solution;

#[test]
fn test_has_valid_path() {
    assert_eq!(
        Solution::has_valid_path(vec![
            vec!['(', '(', '('],
            vec![')', '(', ')'],
            vec!['(', '(', ')'],
            vec!['(', '(', ')']
        ]),
        true
    );
}

#[test]
fn test_has_valid_path2() {
    assert_eq!(
        Solution::has_valid_path(vec![vec![')', ')'], vec!['(', '(']]),
        false
    );
}
