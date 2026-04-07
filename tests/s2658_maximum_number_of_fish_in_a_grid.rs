// Tests for Problem 2658: Maximum Number of Fish in a Grid
// Java reference: src/test/java/g2601_2700/s2658_maximum_number_of_fish_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2658::maximum_number_of_fish_in_a_grid::Solution;

#[test]
fn test_find_max_fish() {
    assert_eq!(
        Solution::find_max_fish(vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0]
        ]),
        7
    );
}

#[test]
fn test_find_max_fish2() {
    assert_eq!(
        Solution::find_max_fish(vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1]
        ]),
        1
    );
}
