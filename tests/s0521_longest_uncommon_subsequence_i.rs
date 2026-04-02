// Tests for Problem 0521: Longest Uncommon Subsequence I
// Java reference: src/test/java/g0501_0600/s0521_longest_uncommon_subsequence_i/SolutionTest.java

use leetcode_in_rust::s0521::longest_uncommon_subsequence_i::Solution;

#[test]
fn test_find_lu_slength() {
    assert_eq!(Solution::find_lu_slength("aba".to_string(), "cdc".to_string()), 3);
}

#[test]
fn test_find_lu_slength2() {
    assert_eq!(Solution::find_lu_slength("aaa".to_string(), "bbb".to_string()), 3);
}

#[test]
fn test_find_lu_slength3() {
    assert_eq!(Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()), -1);
}
