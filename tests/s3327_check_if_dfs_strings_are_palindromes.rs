// Tests for Problem 3327: Check if DFS Strings are Palindromes
// Java reference: src/test/java/g3301_3400/s3327_check_if_dfs_strings_are_palindromes/SolutionTest.java

use leetcode_in_rust::s3327::check_if_dfs_strings_are_palindromes::Solution;

#[test]
fn test_find_answer() {
    assert_eq!(
        Solution::find_answer(vec![-1, 0, 0, 1, 1, 2], "aababa".to_string()),
        vec![true, true, false, true, true, true]
    );
}

#[test]
fn test_find_answer2() {
    assert_eq!(
        Solution::find_answer(vec![-1, 0, 0, 0, 0], "aabcb".to_string()),
        vec![true, true, true, true, true]
    );
}
