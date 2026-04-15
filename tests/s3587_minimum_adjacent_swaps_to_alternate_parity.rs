// Tests for Problem 3587: Minimum Adjacent Swaps to Alternate Parity
// Java reference: src/test/java/g3501_3600/s3587_minimum_adjacent_swaps_to_alternate_parity/SolutionTest.java
use leetcode_in_rust::s3587::minimum_adjacent_swaps_to_alternate_parity::Solution;
#[test] fn test_min_swaps() { assert_eq!(Solution::min_swaps(vec![2, 4, 6, 5, 7]), 3); }
#[test] fn test_min_swaps2() { assert_eq!(Solution::min_swaps(vec![2, 4, 5, 7]), 1); }
#[test] fn test_min_swaps3() { assert_eq!(Solution::min_swaps(vec![1, 2, 3]), 0); }
#[test] fn test_min_swaps4() { assert_eq!(Solution::min_swaps(vec![4, 5, 6, 8]), -1); }
