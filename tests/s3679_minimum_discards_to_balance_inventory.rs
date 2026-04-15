// Tests for Problem 3679: Minimum Discards to Balance Inventory
// Java reference: src/test/java/g3601_3700/s3679_minimum_discards_to_balance_inventory/SolutionTest.java
use leetcode_in_rust::s3679::minimum_discards_to_balance_inventory::Solution;
#[test]
fn test_min_arrivals_to_discard() { assert_eq!(Solution::min_arrivals_to_discard(vec![1, 2, 1, 3, 1], 4, 2), 0); }
#[test]
fn test_min_arrivals_to_discard2() { assert_eq!(Solution::min_arrivals_to_discard(vec![1, 2, 3, 3, 3, 4], 3, 2), 1); }
