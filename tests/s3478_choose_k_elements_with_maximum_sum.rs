// Tests for Problem 3478: Choose K Elements with Maximum Sum
// Java reference: src/test/java/g3401_3500/s3478_choose_k_elements_with_maximum_sum/SolutionTest.java

use leetcode_in_rust::s3478::choose_k_elements_with_maximum_sum::Solution;

#[test]
fn test_find_max_sum() {
    assert_eq!(
        Solution::find_max_sum(vec![4, 2, 1, 5, 3], vec![10, 20, 30, 40, 50], 2),
        vec![80i64, 30, 0, 80, 50]
    );
}

#[test]
fn test_find_max_sum2() {
    assert_eq!(
        Solution::find_max_sum(vec![2, 2, 2, 2], vec![3, 1, 2, 3], 1),
        vec![0i64, 0, 0, 0]
    );
}
