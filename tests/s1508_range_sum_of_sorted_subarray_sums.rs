// Tests for Problem 1508: Range Sum of Sorted Subarray Sums
// Java reference: src/test/java/g1501_1600/s1508_range_sum_of_sorted_subarray_sums/SolutionTest.java

use leetcode_in_rust::s1508::range_sum_of_sorted_subarray_sums::Solution;

#[test]
fn test_range_sum() {
    assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
}

#[test]
fn test_range_sum2() {
    assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4), 6);
}

#[test]
fn test_range_sum3() {
    assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 10), 50);
}
