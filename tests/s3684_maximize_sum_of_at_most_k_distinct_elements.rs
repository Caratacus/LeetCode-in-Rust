// Tests for Problem 3684: Maximize Sum of At Most K Distinct Elements
// Java reference: src/test/java/g3601_3700/s3684_maximize_sum_of_at_most_k_distinct_elements/SolutionTest.java
use leetcode_in_rust::s3684::maximize_sum_of_at_most_k_distinct_elements::Solution;
#[test]
fn test_max_k_distinct() { assert_eq!(Solution::max_k_distinct(vec![84, 93, 100, 77, 90], 3), vec![100, 93, 90]); }
#[test]
fn test_max_k_distinct2() { assert_eq!(Solution::max_k_distinct(vec![84, 93, 100, 77, 93], 3), vec![100, 93, 84]); }
#[test]
fn test_max_k_distinct3() { assert_eq!(Solution::max_k_distinct(vec![1, 1, 1, 2, 2, 2], 6), vec![2, 1]); }
