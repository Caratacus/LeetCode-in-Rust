// Tests for Problem 3002: Maximum Size of a Set After Removals
// Java reference: src/test/java/g3001_3100/s3002_maximum_size_of_a_set_after_removals/SolutionTest.java

use leetcode_in_rust::s3002::maximum_size_of_a_set_after_removals::Solution;

#[test]
fn test_maximum_set_size() {
    assert_eq!(Solution::maximum_set_size(vec![1, 2, 1, 2], vec![1, 1, 1, 1]), 2);
}

#[test]
fn test_maximum_set_size2() {
    assert_eq!(Solution::maximum_set_size(vec![1, 2, 3, 4, 5, 6], vec![2, 3, 2, 3, 2, 3]), 5);
}

#[test]
fn test_maximum_set_size3() {
    assert_eq!(Solution::maximum_set_size(vec![1, 1, 2, 2, 3, 3], vec![4, 4, 5, 5, 6, 6]), 6);
}
