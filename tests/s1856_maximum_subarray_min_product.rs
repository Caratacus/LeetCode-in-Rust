// Tests for Problem 1856: Maximum Subarray Min-Product
// Java reference: src/test/java/g1801_1900/s1856_maximum_subarray_min_product/SolutionTest.java

use leetcode_in_rust::s1856::maximum_subarray_min_product::Solution;

#[test]
fn test_max_sum_min_product() {
    assert_eq!(Solution::max_sum_min_product(vec![1, 2, 3, 2]), 14);
}

#[test]
fn test_max_sum_min_product2() {
    assert_eq!(Solution::max_sum_min_product(vec![2, 3, 3, 1, 2]), 18);
}

#[test]
fn test_max_sum_min_product3() {
    assert_eq!(Solution::max_sum_min_product(vec![3, 1, 5, 6, 4, 2]), 60);
}
