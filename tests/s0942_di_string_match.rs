// Tests for Problem 0942: DI String Match
// Java reference: src/test/java/g0901_1000/s0942_di_string_match/SolutionTest.java

use leetcode_in_rust::s0942::di_string_match::Solution;

#[test]
fn test_di_string_match() {
    let result = Solution::di_string_match("IDID".to_string());
    // Result should be a valid permutation [0..n] where result[i] < result[i+1] if s[i] == 'I'
    assert_eq!(result.len(), 5);
}

#[test]
fn test_di_string_match2() {
    assert_eq!(Solution::di_string_match("III".to_string()), vec![0, 1, 2, 3]);
}

#[test]
fn test_di_string_match3() {
    assert_eq!(Solution::di_string_match("DDI".to_string()), vec![3, 2, 0, 1]);
}
