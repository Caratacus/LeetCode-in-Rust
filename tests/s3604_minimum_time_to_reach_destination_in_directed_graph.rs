// Tests for Problem 3604: Minimum Time to Reach Destination in Directed Graph
// Java reference: src/test/java/g3601_3700/s3604_minimum_time_to_reach_destination_in_directed_graph/SolutionTest.java
use leetcode_in_rust::s3604::minimum_time_to_reach_destination_in_directed_graph::Solution;
#[test] fn test_min_time() { assert_eq!(Solution::min_time(3, vec![vec![0, 1, 0, 1], vec![1, 2, 2, 5]]), 3); }
#[test] fn test_min_time2() { assert_eq!(Solution::min_time(4, vec![vec![0, 1, 0, 3], vec![1, 3, 7, 8], vec![0, 2, 1, 5], vec![2, 3, 4, 7]]), 5); }
#[test] fn test_min_time3() { assert_eq!(Solution::min_time(3, vec![vec![1, 0, 1, 3], vec![1, 2, 3, 5]]), -1); }
#[test] fn test_min_time4() { assert_eq!(Solution::min_time(5, vec![vec![1, 3, 17, 18], vec![1, 3, 0, 7], vec![0, 1, 0, 3], vec![3, 2, 1, 20], vec![1, 2, 25, 25], vec![0, 3, 13, 14], vec![1, 0, 11, 15], vec![0, 4, 19, 21], vec![2, 0, 4, 20]]), 20); }
