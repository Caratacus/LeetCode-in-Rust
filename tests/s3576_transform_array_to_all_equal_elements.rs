// Tests for Problem 3576: Transform Array to All Equal Elements
// Java reference: src/test/java/g3501_3600/s3576_transform_array_to_all_equal_elements/SolutionTest.java
use leetcode_in_rust::s3576::transform_array_to_all_equal_elements::Solution;
#[test] fn test_can_make_equal() { assert_eq!(Solution::can_make_equal(vec![1, -1, 1, -1, 1], 3), true); }
#[test] fn test_can_make_equal2() { assert_eq!(Solution::can_make_equal(vec![-1, -1, -1, 1, 1, 1], 5), false); }
#[test] fn test_can_make_equal3() { assert_eq!(Solution::can_make_equal(vec![1], 3), true); }
