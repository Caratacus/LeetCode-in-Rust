// Tests for Problem 3038: Maximum Number of Operations With the Same Score I
// Java reference: src/test/java/g3001_3100/s3038_maximum_number_of_operations_with_the_same_score_i/SolutionTest.java

use leetcode_in_rust::s3038::maximum_number_of_operations_with_the_same_score_i::Solution;

#[test]
fn test_max_operations() {
    assert_eq!(Solution::max_operations(vec![3, 2, 1, 4, 5]), 2);
}

#[test]
fn test_max_operations2() {
    assert_eq!(Solution::max_operations(vec![3, 2, 6, 1, 4]), 1);
}
