// Tests for Problem 3588: Find Maximum Area of a Triangle
// Java reference: src/test/java/g3501_3600/s3588_find_maximum_area_of_a_triangle/SolutionTest.java
use leetcode_in_rust::s3588::find_maximum_area_of_a_triangle::Solution;
#[test] fn test_max_area() { assert_eq!(Solution::max_area(vec![vec![1, 1], vec![1, 2], vec![3, 2], vec![3, 3]]), 2i64); }
#[test] fn test_max_area2() { assert_eq!(Solution::max_area(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), -1i64); }
