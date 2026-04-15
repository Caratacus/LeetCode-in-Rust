// Tests for Problem 3572: Maximize YSum by Picking a Triplet of Distinct XValues
// Java reference: src/test/java/g3501_3600/s3572_maximize_ysum_by_picking_a_triplet_of_distinct_xvalues/SolutionTest.java
use leetcode_in_rust::s3572::maximize_ysum_by_picking_a_triplet_of_distinct_xvalues::Solution;
#[test] fn test_max_sum_distinct_triplet() { assert_eq!(Solution::max_sum_distinct_triplet(vec![1, 2, 1, 3, 2], vec![5, 3, 4, 6, 2]), 14); }
#[test] fn test_max_sum_distinct_triplet2() { assert_eq!(Solution::max_sum_distinct_triplet(vec![1, 2, 1, 2], vec![4, 5, 6, 7]), -1); }
