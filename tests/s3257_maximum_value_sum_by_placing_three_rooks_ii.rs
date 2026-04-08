// Tests for Problem 3257: Maximum Value Sum by Placing Three Rooks II
// Java reference: src/test/java/g3201_3300/s3257_maximum_value_sum_by_placing_three_rooks_ii/SolutionTest.java

use leetcode_in_rust::s3257::maximum_value_sum_by_placing_three_rooks_ii::Solution;

#[test]
fn test_maximum_value_sum() {
    assert_eq!(
        Solution::maximum_value_sum(vec![vec![-3, 1, 1, 1], vec![-3, 1, -3, 1], vec![-3, 2, 1, 1]]),
        4
    );
}

#[test]
fn test_maximum_value_sum2() {
    assert_eq!(
        Solution::maximum_value_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        15
    );
}

#[test]
fn test_maximum_value_sum3() {
    assert_eq!(
        Solution::maximum_value_sum(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]),
        3
    );
}
