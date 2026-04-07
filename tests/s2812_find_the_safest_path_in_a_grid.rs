// Tests for Problem 2812: Find the Safest Path in a Grid
// Java reference: src/test/java/g2801_2900/s2812_find_the_safest_path_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2812::find_the_safest_path_in_a_grid::Solution;

#[test]
fn test_maximum_safeness_factor() {
    assert_eq!(
        Solution::maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
        0
    );
}

#[test]
fn test_maximum_safeness_factor2() {
    assert_eq!(
        Solution::maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]),
        2
    );
}

#[test]
fn test_maximum_safeness_factor3() {
    assert_eq!(
        Solution::maximum_safeness_factor(vec![
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 0]
        ]),
        2
    );
}
