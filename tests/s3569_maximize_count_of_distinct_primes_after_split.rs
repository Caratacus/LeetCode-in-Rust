// Tests for Problem 3569: Maximize Count of Distinct Primes After Split
// Java reference: src/test/java/g3501_3600/s3569_maximize_count_of_distinct_primes_after_split/SolutionTest.java
use leetcode_in_rust::s3569::maximize_count_of_distinct_primes_after_split::Solution;
#[test] fn test_maximum_count() { assert_eq!(Solution::maximum_count(vec![2, 1, 3, 1, 2], vec![vec![1, 2], vec![3, 3]]), vec![3, 4]); }
#[test] fn test_maximum_count2() { assert_eq!(Solution::maximum_count(vec![2, 1, 4], vec![vec![0, 1]]), vec![0]); }
#[test] fn test_maximum_count3() { assert_eq!(Solution::maximum_count(vec![2, 34], vec![vec![1, 2], vec![1, 3]]), vec![2, 2]); }
#[test] fn test_maximum_count4() { assert_eq!(Solution::maximum_count(vec![4, 2], vec![vec![0, 2], vec![0, 2]]), vec![2, 2]); }
