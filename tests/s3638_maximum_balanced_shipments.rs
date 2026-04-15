// Tests for Problem 3638: Maximum Balanced Shipments
// Java reference: src/test/java/g3601_3700/s3638_maximum_balanced_shipments/SolutionTest.java
use leetcode_in_rust::s3638::maximum_balanced_shipments::Solution;
#[test]
fn test_max_balanced_shipments() { assert_eq!(Solution::max_balanced_shipments(vec![2, 5, 1, 4, 3]), 2); }
#[test]
fn test_max_balanced_shipments2() { assert_eq!(Solution::max_balanced_shipments(vec![4, 4]), 0); }
