// Tests for Problem 0522: Longest Uncommon Subsequence II
// Java reference: src/test/java/g0501_0600/s0522_longest_uncommon_subsequence_ii/SolutionTest.java

use leetcode_in_rust::s0522::longest_uncommon_subsequence_ii::Solution;

#[test]
fn test_find_lu_slength() {
    assert_eq!(
        Solution::find_lu_slength(vec!["aba".to_string(), "cdc".to_string(), "eae".to_string()]),
        3
    );
}

#[test]
fn test_find_lu_slength2() {
    assert_eq!(
        Solution::find_lu_slength(vec!["aaa".to_string(), "aaa".to_string(), "aa".to_string()]),
        -1
    );
}
