// Tests for Problem 3197: Find the Minimum Area to Cover All Ones II
// Java reference: src/test/java/g3101_3200/s3197_find_the_minimum_area_to_cover_all_ones_ii/SolutionTest.java

use leetcode_in_rust::s3197::find_the_minimum_area_to_cover_all_ones_ii::Solution;

#[test]
fn test_minimum_sum() {
    assert_eq!(Solution::minimum_sum(vec![vec![1, 0, 1], vec![1, 1, 1]]), 5);
}

#[test]
fn test_minimum_sum2() {
    assert_eq!(Solution::minimum_sum(vec![vec![1, 0, 1, 0], vec![0, 1, 0, 1]]), 5);
}
