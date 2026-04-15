// Tests for Problem 3594: Minimum Time to Transport All Individuals
// Java reference: src/test/java/g3501_3600/s3594_minimum_time_to_transport_all_individuals/SolutionTest.java
use leetcode_in_rust::s3594::minimum_time_to_transport_all_individuals::Solution;
#[test] fn test_min_time() { let r = Solution::min_time(1, 1, 2, vec![5], vec![1.0, 1.3]); assert!((r - 5.0).abs() < 1e-5); }
#[test] fn test_min_time2() { let r = Solution::min_time(3, 2, 3, vec![2, 5, 8], vec![1.0, 1.5, 0.75]); assert!((r - 14.5).abs() < 1e-5); }
#[test] fn test_min_time3() { let r = Solution::min_time(2, 1, 2, vec![10, 10], vec![2.0, 2.0]); assert!((r - (-1.0)).abs() < 1e-5); }
