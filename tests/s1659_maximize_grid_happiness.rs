// Tests for Problem 1659: Maximize Grid Happiness
// Java reference: src/test/java/g1601_1700/s1659_maximize_grid_happiness/SolutionTest.java

use leetcode_in_rust::s1659::maximize_grid_happiness::Solution;

#[test]
fn test_get_max_grid_happiness() {
    assert_eq!(Solution::get_max_grid_happiness(2, 3, 1, 2), 240);
}

#[test]
fn test_get_max_grid_happiness2() {
    assert_eq!(Solution::get_max_grid_happiness(3, 1, 2, 1), 260);
}

#[test]
fn test_get_max_grid_happiness3() {
    assert_eq!(Solution::get_max_grid_happiness(2, 2, 4, 0), 240);
}
