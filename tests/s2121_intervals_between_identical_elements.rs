// Tests for Problem 2121: Intervals Between Identical Elements
// Java reference: src/test/java/g2101_2200/s2121_intervals_between_identical_elements/SolutionTest.java

use leetcode_in_rust::s2121::intervals_between_identical_elements::Solution;

#[test]
fn test_get_distances() {
    assert_eq!(
        Solution::get_distances(vec![2, 1, 3, 1, 2, 3, 3]),
        vec![4, 2, 7, 2, 4, 4, 5]
    );
}

#[test]
fn test_get_distances2() {
    assert_eq!(
        Solution::get_distances(vec![10, 5, 10, 10]),
        vec![5, 0, 3, 4]
    );
}
