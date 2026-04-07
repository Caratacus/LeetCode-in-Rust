// Tests for Problem 2258: Escape the Spreading Fire
// Java reference: src/test/java/g2201_2300/s2258_escape_the_spreading_fire/SolutionTest.java

use leetcode_in_rust::s2258::escape_the_spreading_fire::Solution;

#[test]
fn test_maximum_minutes() {
    assert_eq!(
        Solution::maximum_minutes(vec![
            vec![0, 2, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 2, 1, 0],
            vec![0, 2, 0, 0, 1, 2, 0],
            vec![0, 0, 2, 2, 2, 0, 2],
            vec![0, 0, 0, 0, 0, 0, 0]
        ]),
        3
    );
}

#[test]
fn test_maximum_minutes2() {
    assert_eq!(
        Solution::maximum_minutes(vec![vec![0, 0, 0, 0], vec![0, 1, 2, 0], vec![0, 2, 0, 0]]),
        -1
    );
}

#[test]
fn test_maximum_minutes3() {
    assert_eq!(
        Solution::maximum_minutes(vec![vec![0, 0, 0], vec![2, 2, 0], vec![1, 2, 0]]),
        1000000000
    );
}
