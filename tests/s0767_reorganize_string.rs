// Tests for Problem 0767: Reorganize String
// Java reference: src/test/java/g0701_0800/s0767_reorganize_string/SolutionTest.java

use leetcode_in_rust::s0767::reorganize_string::Solution;

#[test]
fn test_reorganize_string() {
    assert_eq!(Solution::reorganize_string("aab".to_string()), "aba");
}

#[test]
fn test_reorganize_string2() {
    assert_eq!(Solution::reorganize_string("aaab".to_string()), "");
}

#[test]
fn test_reorganize_string3() {
    assert_eq!(Solution::reorganize_string("aaabbbb".to_string()), "bababab");
}

#[test]
fn test_reorganize_string4() {
    assert_eq!(Solution::reorganize_string("vvvlo".to_string()), "vlvov");
}
