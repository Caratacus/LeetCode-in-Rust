// Tests for Problem 2132: Stamping the Grid
// Java reference: src/test/java/g2101_2200/s2132_stamping_the_grid/SolutionTest.java

use leetcode_in_rust::s2132::stamping_the_grid::Solution;

#[test]
fn test_possible_to_stamp() {
    assert_eq!(
        Solution::possible_to_stamp(
            vec![
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0]
            ],
            4,
            3
        ),
        true
    );
}

#[test]
fn test_possible_to_stamp2() {
    assert_eq!(
        Solution::possible_to_stamp(
            vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]],
            2,
            2
        ),
        false
    );
}

#[test]
fn test_possible_to_stamp3() {
    assert_eq!(
        Solution::possible_to_stamp(
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0]
            ],
            1,
            1
        ),
        true
    );
}

#[test]
fn test_possible_to_stamp4() {
    assert_eq!(
        Solution::possible_to_stamp(
            vec![vec![0], vec![0], vec![0], vec![0], vec![1], vec![1], vec![0], vec![0], vec![1]],
            9,
            1
        ),
        false
    );
}

#[test]
fn test_possible_to_stamp5() {
    assert_eq!(
        Solution::possible_to_stamp(vec![vec![1], vec![1], vec![0], vec![0]], 3, 1),
        false
    );
}

#[test]
fn test_possible_to_stamp6() {
    assert_eq!(
        Solution::possible_to_stamp(vec![vec![0, 1], vec![0, 0]], 2, 2),
        false
    );
}

#[test]
fn test_possible_to_stamp7() {
    assert_eq!(
        Solution::possible_to_stamp(vec![vec![1, 1, 0, 0]], 1, 3),
        false
    );
}
