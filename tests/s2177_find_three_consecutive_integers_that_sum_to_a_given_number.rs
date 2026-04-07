// Tests for Problem 2177: Find Three Consecutive Integers That Sum to a Given Number
// Java reference: src/test/java/g2101_2200/s2177_find_three_consecutive_integers_that_sum_to_a_given_number/SolutionTest.java

use leetcode_in_rust::s2177::find_three_consecutive_integers_that_sum_to_a_given_number::Solution;

#[test]
fn test_sum_of_three() {
    assert_eq!(Solution::sum_of_three(33), vec![10, 11, 12]);
}

#[test]
fn test_sum_of_three2() {
    assert_eq!(Solution::sum_of_three(4), vec![]);
}
