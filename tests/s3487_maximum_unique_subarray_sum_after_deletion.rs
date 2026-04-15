// Tests for Problem 3487: Maximum Unique Subarray Sum After Deletion
// Java reference: src/test/java/g3401_3500/s3487_maximum_unique_subarray_sum_after_deletion/SolutionTest.java

use leetcode_in_rust::s3487::maximum_unique_subarray_sum_after_deletion::Solution;

#[test]
fn test_max_sum() {
    assert_eq!(Solution::max_sum(vec![1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_max_sum2() {
    assert_eq!(Solution::max_sum(vec![1, 1, 0, 1, 1]), 1);
}

#[test]
fn test_max_sum3() {
    assert_eq!(Solution::max_sum(vec![1, 2, -1, -2, 1, 0, -1]), 3);
}

#[test]
fn test_max_sum4() {
    assert_eq!(Solution::max_sum(vec![-100]), -100);
}
