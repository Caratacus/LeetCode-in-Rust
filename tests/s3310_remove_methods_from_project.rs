// Tests for Problem 3310: Remove Methods From Project
// Java reference: src/test/java/g3301_3400/s3310_remove_methods_from_project/SolutionTest.java

use leetcode_in_rust::s3310::remove_methods_from_project::Solution;

#[test]
fn test_remaining_methods() {
    assert_eq!(
        Solution::remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
}

#[test]
fn test_remaining_methods2() {
    assert_eq!(
        Solution::remaining_methods(5, 0, vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]]),
        vec![3, 4]
    );
}

#[test]
fn test_remaining_methods3() {
    assert_eq!(
        Solution::remaining_methods(3, 2, vec![vec![1, 2], vec![0, 1], vec![2, 0]]),
        vec![]
    );
}
