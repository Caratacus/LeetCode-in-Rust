// Tests for Problem 1498: Number of Subsequences That Satisfy the Given Sum Condition
// Java reference: src/test/java/g1401_1500/s1498_number_of_subsequences_that_satisfy_the_given_sum_condition/SolutionTest.java

use leetcode_in_rust::s1498::number_of_subsequences_that_satisfy_the_given_sum_condition::Solution;

#[test]
fn test_num_subseq() {
    assert_eq!(Solution::num_subseq(vec![3, 5, 6, 7], 9), 4);
}

#[test]
fn test_num_subseq2() {
    assert_eq!(Solution::num_subseq(vec![3, 3, 6, 8], 10), 6);
}

#[test]
fn test_num_subseq3() {
    assert_eq!(Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
}
