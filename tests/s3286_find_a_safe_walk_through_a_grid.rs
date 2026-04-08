// Tests for Problem 3286: Find a Safe Walk Through a Grid
// Java reference: src/test/java/g3201_3300/s3286_find_a_safe_walk_through_a_grid/SolutionTest.java

use leetcode_in_rust::s3286::find_a_safe_walk_through_a_grid::Solution;

#[test]
fn test_find_safe_walk() {
    assert_eq!(
        Solution::find_safe_walk(
            vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ],
            1
        ),
        true
    );
}

#[test]
fn test_find_safe_walk2() {
    assert_eq!(
        Solution::find_safe_walk(
            vec![
                vec![0, 1, 1, 0, 0, 0],
                vec![1, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 0, 1],
                vec![0, 0, 1, 0, 1, 0]
            ],
            3
        ),
        false
    );
}

#[test]
fn test_find_safe_walk3() {
    assert_eq!(
        Solution::find_safe_walk(
            vec![
                vec![1, 1, 1],
                vec![1, 0, 1],
                vec![1, 1, 1]
            ],
            5
        ),
        true
    );
}
