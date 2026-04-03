// Tests for Problem 1411: Number of Ways to Paint N × 3 Grid
// Java reference: src/test/java/g1401_1500/s1411_number_of_ways_to_paint_n_3_grid/SolutionTest.java

use leetcode_in_rust::s1411::number_of_ways_to_paint_n_3_grid::Solution;

#[test]
fn test_num_of_ways() {
    assert_eq!(Solution::num_of_ways(1), 12);
}

#[test]
fn test_num_of_ways2() {
    assert_eq!(Solution::num_of_ways(5000), 30228214);
}
