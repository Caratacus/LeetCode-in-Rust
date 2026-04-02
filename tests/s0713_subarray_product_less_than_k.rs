// Tests for Problem 0713: Subarray Product Less Than K
// Java reference: src/test/java/g0701_0800/s0713_subarray_product_less_than_k/SolutionTest.java

use leetcode_in_rust::s0713::subarray_product_less_than_k::Solution;

#[test]
fn test_num_subarray_product_less_than_k() {
    assert_eq!(Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
}

#[test]
fn test_num_subarray_product_less_than_k2() {
    assert_eq!(Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
}
