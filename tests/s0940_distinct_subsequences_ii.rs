// Tests for Problem 0940: Distinct Subsequences II
// Java reference: src/test/java/g0901_1000/s0940_distinct_subsequences_ii/SolutionTest.java

use leetcode_in_rust::s0940::distinct_subsequences_ii::Solution;

#[test]
fn test_distinct_subseq_ii() {
    let result = Solution::distinct_subseq_ii("abc".to_string());
    assert_eq!(result, 7);
}

#[test]
fn test_distinct_subseq_ii2() {
    let result = Solution::distinct_subseq_ii("aba".to_string());
    assert_eq!(result, 6);
}

#[test]
fn test_distinct_subseq_ii3() {
    let result = Solution::distinct_subseq_ii("aaa".to_string());
    assert_eq!(result, 3);
}
