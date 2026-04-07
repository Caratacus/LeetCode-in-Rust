// Tests for Problem 2088: Count Fertile Pyramids in a Land
// Java reference: src/test/java/g2001_2100/s2088_count_fertile_pyramids_in_a_land/SolutionTest.java

use leetcode_in_rust::s2088::count_fertile_pyramids_in_a_land::Solution;

#[test]
fn test_count_pyramids() {
    assert_eq!(
        Solution::count_pyramids(vec![
            vec![0, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0]
        ]),
        13
    );
}

#[test]
fn test_count_pyramids2() {
    assert_eq!(
        Solution::count_pyramids(vec![
            vec![1, 1, 1],
            vec![1, 1, 1]
        ]),
        2
    );
}

#[test]
fn test_count_pyramids3() {
    assert_eq!(
        Solution::count_pyramids(vec![
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0]
        ]),
        12
    );
}
