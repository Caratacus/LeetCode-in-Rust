// Tests for Problem 3122: Minimum Number of Operations to Satisfy Conditions
// Java reference: src/test/java/g3101_3200/s3122_minimum_number_of_operations_to_satisfy_conditions/SolutionTest.java

use leetcode_in_rust::s3122::minimum_number_of_operations_to_satisfy_conditions::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(
        Solution::minimum_operations(vec![vec![1, 0, 2], vec![1, 0, 2]]),
        0
    );
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(
        Solution::minimum_operations(vec![vec![1, 1, 1], vec![0, 0, 0]]),
        3
    );
}
