// Tests for Problem 1147: Longest Chunked Palindrome Decomposition
// Java reference: src/test/java/g1101_1200/s1147_longest_chunked_palindrome_decomposition/SolutionTest.java

use leetcode_in_rust::s1147::longest_chunked_palindrome_decomposition::Solution;

#[test]
fn test_longest_decomposition() {
    assert_eq!(
        Solution::longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string()),
        7
    );
}

#[test]
fn test_longest_decomposition2() {
    assert_eq!(Solution::longest_decomposition("merchant".to_string()), 1);
}

#[test]
fn test_longest_decomposition3() {
    assert_eq!(
        Solution::longest_decomposition("antaprezatepzapreanta".to_string()),
        11
    );
}
