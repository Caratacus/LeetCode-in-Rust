// Tests for Problem 0891: Sum of Subsequence Widths
// Java reference: src/test/java/g0801_0900/s0891_sum_of_subsequence_widths/SolutionTest.java

use leetcode_in_rust::s0891::sum_of_subsequence_widths::Solution;

#[test]
fn test_sum_subseq_widths() {
    let result = Solution::sum_subseq_widths(vec![2, 1, 3]);
    assert_eq!(result, 6);
}

#[test]
fn test_sum_subseq_widths2() {
    let result = Solution::sum_subseq_widths(vec![2]);
    assert_eq!(result, 0);
}
