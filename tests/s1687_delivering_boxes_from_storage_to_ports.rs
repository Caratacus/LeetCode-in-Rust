// Tests for Problem 1687: Delivering Boxes from Storage to Ports
// Java reference: src/test/java/g1601_1700/s1687_delivering_boxes_from_storage_to_ports/SolutionTest.java

use leetcode_in_rust::s1687::delivering_boxes_from_storage_to_ports::Solution;

#[test]
fn test_box_delivering() {
    assert_eq!(
        Solution::box_delivering(vec![vec![1, 1], vec![2, 1], vec![1, 1]], 2, 3, 3),
        4
    );
}

#[test]
fn test_box_delivering2() {
    assert_eq!(
        Solution::box_delivering(
            vec![vec![1, 2], vec![3, 3], vec![3, 1], vec![3, 1], vec![2, 4]],
            3,
            3,
            6
        ),
        6
    );
}

#[test]
fn test_box_delivering3() {
    assert_eq!(
        Solution::box_delivering(
            vec![vec![1, 4], vec![1, 2], vec![2, 1], vec![2, 1], vec![3, 2], vec![3, 4]],
            3,
            6,
            7
        ),
        6
    );
}
