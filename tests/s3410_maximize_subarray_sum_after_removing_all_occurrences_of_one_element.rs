// Tests for Problem 3410: Maximize Subarray Sum After Removing All Occurrences of One Element
// Java reference: src/test/java/g3401_3500/s3410_maximize_subarray_sum_after_removing_all_occurrences_of_one_element/SolutionTest.java

use leetcode_in_rust::s3410::maximize_subarray_sum_after_removing_all_occurrences_of_one_element::Solution;

#[test]
fn test_max_subarray_sum() {
    assert_eq!(Solution::max_subarray_sum(vec![-3, 2, -2, -1, 3, -2, 3]), 7i64);
}

#[test]
fn test_max_subarray_sum2() {
    assert_eq!(Solution::max_subarray_sum(vec![1, 2, 3, 4]), 10i64);
}
