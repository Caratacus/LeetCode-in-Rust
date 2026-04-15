// Tests for Problem 3452: Sum of Good Numbers
// Java reference: src/test/java/g3401_3500/s3452_sum_of_good_numbers/SolutionTest.java

use leetcode_in_rust::s3452::sum_of_good_numbers::Solution;

#[test]
fn test_sum_of_good_numbers() {
    assert_eq!(Solution::sum_of_good_numbers(vec![1, 3, 2, 1, 5, 4], 2), 12);
}

#[test]
fn test_sum_of_good_numbers2() {
    assert_eq!(Solution::sum_of_good_numbers(vec![2, 1], 1), 2);
}
