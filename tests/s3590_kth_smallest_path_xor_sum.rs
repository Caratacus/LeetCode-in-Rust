// Tests for Problem 3590: Kth Smallest Path XOR Sum
// Java reference: src/test/java/g3501_3600/s3590_kth_smallest_path_xor_sum/SolutionTest.java
use leetcode_in_rust::s3590::kth_smallest_path_xor_sum::Solution;
#[test] fn test_kth_smallest() { assert_eq!(Solution::kth_smallest(vec![-1, 0, 0], vec![1, 1, 1], vec![vec![0, 1], vec![0, 2], vec![0, 3]]), vec![0, 1, -1]); }
#[test] fn test_kth_smallest2() { assert_eq!(Solution::kth_smallest(vec![-1, 0, 1], vec![5, 2, 7], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 1]]), vec![0, 7, -1, 0]); }
