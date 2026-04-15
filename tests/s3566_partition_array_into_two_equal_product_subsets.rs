// Tests for Problem 3566: Partition Array Into Two Equal Product Subsets
// Java reference: src/test/java/g3501_3600/s3566_partition_array_into_two_equal_product_subsets/SolutionTest.java
use leetcode_in_rust::s3566::partition_array_into_two_equal_product_subsets::Solution;
#[test] fn test_check_equal_partitions() { assert_eq!(Solution::check_equal_partitions(vec![3, 1, 6, 8, 4], 24), true); }
#[test] fn test_check_equal_partitions2() { assert_eq!(Solution::check_equal_partitions(vec![2, 5, 3, 7], 15), false); }
