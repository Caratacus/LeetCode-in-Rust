// Tests for Problem 1048: Longest String Chain
// Java reference: src/test/java/g1001_1100/s1048_longest_string_chain/SolutionTest.java

use leetcode_in_rust::s1048::longest_string_chain::Solution;

#[test]
fn test_longest_str_chain() {
    assert_eq!(
        Solution::longest_str_chain(vec!["a".to_string(), "b".to_string(), "ba".to_string(), "bca".to_string(), "bda".to_string(), "bdca".to_string()]),
        4
    );
}

#[test]
fn test_longest_str_chain2() {
    assert_eq!(
        Solution::longest_str_chain(vec!["xbc".to_string(), "pcxbcf".to_string(), "xb".to_string(), "cxbc".to_string(), "pcxbc".to_string()]),
        5
    );
}

#[test]
fn test_longest_str_chain3() {
    assert_eq!(
        Solution::longest_str_chain(vec!["abcd".to_string(), "dbqca".to_string()]),
        1
    );
}
