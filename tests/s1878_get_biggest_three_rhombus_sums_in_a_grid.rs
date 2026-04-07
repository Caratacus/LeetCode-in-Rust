// Tests for Problem 1878: Get Biggest Three Rhombus Sums in a Grid
// Java reference: src/test/java/g1801_1900/s1878_get_biggest_three_rhombus_sums_in_a_grid/SolutionTest.java

use leetcode_in_rust::s1878::get_biggest_three_rhombus_sums_in_a_grid::Solution;

#[test]
fn test_get_biggest_three() {
    assert_eq!(
        Solution::get_biggest_three(vec![
            vec![3, 4, 5, 1, 3],
            vec![3, 4, 1, 5, 8]
        ]),
        vec![11, 10, 8]
    );
}

#[test]
fn test_get_biggest_three2() {
    assert_eq!(
        Solution::get_biggest_three(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]),
        vec![20, 13, 12]
    );
}
