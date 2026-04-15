// Tests for Problem 3560: Find Minimum Log Transportation Cost
// Java reference: src/test/java/g3501_3600/s3560_find_minimum_log_transportation_cost/SolutionTest.java
use leetcode_in_rust::s3560::find_minimum_log_transportation_cost::Solution;
#[test] fn test_min_cutting_cost() { assert_eq!(Solution::min_cutting_cost(6, 5, 5), 5i64); }
#[test] fn test_min_cutting_cost2() { assert_eq!(Solution::min_cutting_cost(4, 4, 6), 0i64); }
#[test] fn test_min_cutting_cost3() { assert_eq!(Solution::min_cutting_cost(0, 5, 3), 0i64); }
#[test] fn test_min_cutting_cost8() { assert_eq!(Solution::min_cutting_cost(2, 5, 2), 6i64); }
#[test] fn test_min_cutting_cost9() { assert_eq!(Solution::min_cutting_cost(1, 10, 9), 9i64); }
