// Tests for Problem 3578: Count Partitions with Max Min Difference at Most K
// Java reference: src/test/java/g3501_3600/s3578_count_partitions_with_max_min_difference_at_most_k/SolutionTest.java
use leetcode_in_rust::s3578::count_partitions_with_max_min_difference_at_most_k::Solution;
#[test] fn test_count_partitions() { assert_eq!(Solution::count_partitions(vec![9, 4, 1, 3, 7], 4), 6); }
#[test] fn test_count_partitions2() { assert_eq!(Solution::count_partitions(vec![3, 3, 4], 0), 2); }
