// Tests for Problem 3668: Restore Finishing Order
// Java reference: src/test/java/g3601_3700/s3668_restore_finishing_order/SolutionTest.java
use leetcode_in_rust::s3668::restore_finishing_order::Solution;
#[test]
fn test_recover_order() { assert_eq!(Solution::recover_order(vec![3, 1, 2, 5, 4], vec![1, 3, 4]), vec![3, 1, 4]); }
#[test]
fn test_recover_order2() { assert_eq!(Solution::recover_order(vec![1, 4, 5, 3, 2], vec![2, 5]), vec![5, 2]); }
