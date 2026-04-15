// Tests for Problem 3462: Maximum Sum with at Most K Elements
// Java reference: src/test/java/g3401_3500/s3462_maximum_sum_with_at_most_k_elements/SolutionTest.java

use leetcode_in_rust::s3462::maximum_sum_with_at_most_k_elements::Solution;

#[test]
fn test_max_sum() {
    assert_eq!(Solution::max_sum(vec![vec![1, 2], vec![3, 4]], vec![1, 2], 2), 7i64);
}

#[test]
fn test_max_sum2() {
    assert_eq!(Solution::max_sum(vec![vec![5, 3, 7], vec![8, 2, 6]], vec![2, 2], 3), 21i64);
}

#[test]
fn test_max_sum3() {
    assert_eq!(Solution::max_sum(vec![], vec![2, 2], 3), 0i64);
}
