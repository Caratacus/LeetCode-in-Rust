// Tests for Problem 3584: Maximum Product of First and Last Elements of a Subsequence
// Java reference: src/test/java/g3501_3600/s3584_maximum_product_of_first_and_last_elements_of_a_subsequence/SolutionTest.java
use leetcode_in_rust::s3584::maximum_product_of_first_and_last_elements_of_a_subsequence::Solution;
#[test] fn test_maximum_product() { assert_eq!(Solution::maximum_product(vec![-1, -9, 2, 3, -2, -3, 1], 1), 81i64); }
#[test] fn test_maximum_product2() { assert_eq!(Solution::maximum_product(vec![1, 3, -5, 5, 6, -4], 3), 20i64); }
#[test] fn test_maximum_product3() { assert_eq!(Solution::maximum_product(vec![2, -1, 2, -6, 5, 2, -5, 7], 2), 35i64); }
