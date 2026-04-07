// Tests for Problem 2167: Minimum Time to Remove All Cars Containing Illegal Goods
// Java reference: src/test/java/g2101_2200/s2167_minimum_time_to_remove_all_cars_containing_illegal_goods/SolutionTest.java

use leetcode_in_rust::s2167::minimum_time_to_remove_all_cars_containing_illegal_goods::Solution;

#[test]
fn test_minimum_time() {
    assert_eq!(Solution::minimum_time("0010".to_string()), 2);
}

#[test]
fn test_minimum_time2() {
    assert_eq!(Solution::minimum_time("1100101".to_string()), 5);
}
