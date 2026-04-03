// Tests for Problem 1380: Lucky Numbers in a Matrix
// Java reference: src/test/java/g1301_1400/s1380_lucky_numbers_in_a_matrix/SolutionTest.java

use leetcode_in_rust::s1380::lucky_numbers_in_a_matrix::Solution;

#[test]
fn test_lucky_numbers() {
    assert_eq!(
        Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
        vec![15]
    );
}

#[test]
fn test_lucky_numbers2() {
    assert_eq!(
        Solution::lucky_numbers(vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]]),
        vec![12]
    );
}

#[test]
fn test_lucky_numbers3() {
    assert_eq!(
        Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
        vec![7]
    );
}
