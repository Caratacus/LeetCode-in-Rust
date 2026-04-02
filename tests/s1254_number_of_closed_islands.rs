// Tests for Problem 1254: Number of Closed Islands
// Java reference: src/test/java/g1201_1300/s1254_number_of_closed_islands/SolutionTest.java

use leetcode_in_rust::s1254::number_of_closed_islands::Solution;

#[test]
fn test_closed_island() {
    assert_eq!(
        Solution::closed_island(vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0]
        ]),
        2
    );
}

#[test]
fn test_closed_island2() {
    assert_eq!(
        Solution::closed_island(vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0]
        ]),
        1
    );
}

#[test]
fn test_closed_island3() {
    assert_eq!(
        Solution::closed_island(vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1]
        ]),
        2
    );
}
