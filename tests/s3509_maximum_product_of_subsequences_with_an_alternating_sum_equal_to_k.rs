// Tests for Problem 3509: Maximum Product of Subsequences with an Alternating Sum Equal to K
// Java reference: src/test/java/g3501_3600/s3509_maximum_product_of_subsequences_with_an_alternating_sum_equal_to_k/SolutionTest.java

use leetcode_in_rust::s3509::maximum_product_of_subsequences_with_an_alternating_sum_equal_to_k::Solution;

#[test]
fn test_max_product() {
    assert_eq!(Solution::max_product(vec![1, 2, 3], 2, 10), 6);
}

#[test]
fn test_max_product2() {
    assert_eq!(Solution::max_product(vec![0, 2, 3], -5, 12), -1);
}

#[test]
fn test_max_product3() {
    assert_eq!(Solution::max_product(vec![2, 2, 3, 3], 0, 9), 9);
}

#[test]
fn test_max_product4() {
    assert_eq!(Solution::max_product(vec![12, 0, 9], 21, 20), 0);
}
