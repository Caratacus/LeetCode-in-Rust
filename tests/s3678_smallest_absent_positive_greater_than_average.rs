// Tests for Problem 3678: Smallest Absent Positive Greater Than Average
// Java reference: src/test/java/g3601_3700/s3678_smallest_absent_positive_greater_than_average/SolutionTest.java
use leetcode_in_rust::s3678::smallest_absent_positive_greater_than_average::Solution;
#[test]
fn test_smallest_absent() { assert_eq!(Solution::smallest_absent(vec![3, 5]), 6); }
#[test]
fn test_smallest_absent2() { assert_eq!(Solution::smallest_absent(vec![-1, 1, 2]), 3); }
#[test]
fn test_smallest_absent3() { assert_eq!(Solution::smallest_absent(vec![4, -1]), 2); }
#[test]
fn test_smallest_absent4() { assert_eq!(Solution::smallest_absent(vec![-2, -1]), 1); }
