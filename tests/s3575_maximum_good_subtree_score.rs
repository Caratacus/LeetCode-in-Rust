// Tests for Problem 3575: Maximum Good Subtree Score
// Java reference: src/test/java/g3501_3600/s3575_maximum_good_subtree_score/SolutionTest.java
use leetcode_in_rust::s3575::maximum_good_subtree_score::Solution;
#[test] fn test_good_subtree_sum() { assert_eq!(Solution::good_subtree_sum(vec![2, 3], vec![-1, 0]), 8); }
#[test] fn test_good_subtree_sum2() { assert_eq!(Solution::good_subtree_sum(vec![1, 5, 2], vec![-1, 0, 0]), 15); }
#[test] fn test_good_subtree_sum3() { assert_eq!(Solution::good_subtree_sum(vec![34, 1, 2], vec![-1, 0, 1]), 42); }
#[test] fn test_good_subtree_sum4() { assert_eq!(Solution::good_subtree_sum(vec![3, 22, 5], vec![-1, 0, 1]), 18); }
