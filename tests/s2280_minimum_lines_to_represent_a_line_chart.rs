// Tests for Problem 2280: Minimum Lines to Represent a Line Chart
// Java reference: src/test/java/g2201_2300/s2280_minimum_lines_to_represent_a_line_chart/SolutionTest.java

use leetcode_in_rust::s2280::minimum_lines_to_represent_a_line_chart::Solution;

#[test]
fn test_minimum_lines() {
    assert_eq!(
        Solution::minimum_lines(vec![
            vec![1, 7],
            vec![2, 6],
            vec![3, 5],
            vec![4, 4],
            vec![5, 4],
            vec![6, 3],
            vec![7, 2],
            vec![8, 1]
        ]),
        3
    );
}

#[test]
fn test_minimum_lines2() {
    assert_eq!(
        Solution::minimum_lines(vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]]),
        1
    );
}

#[test]
fn test_minimum_lines3() {
    assert_eq!(Solution::minimum_lines(vec![vec![3, 4]]), 0);
}
