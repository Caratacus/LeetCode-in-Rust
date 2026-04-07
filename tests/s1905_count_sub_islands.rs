// Tests for Problem 1905: Count Sub Islands
// Java reference: src/test/java/g1901_2000/s1905_count_sub_islands/SolutionTest.java

use leetcode_in_rust::s1905::count_sub_islands::Solution;

#[test]
fn test_count_sub_islands() {
    assert_eq!(
        Solution::count_sub_islands(
            vec![
                vec![1, 1, 1, 0, 0],
                vec![0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 1, 1]
            ],
            vec![
                vec![1, 1, 1, 0, 0],
                vec![0, 0, 1, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![1, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0]
            ]
        ),
        3
    );
}

#[test]
fn test_count_sub_islands2() {
    assert_eq!(
        Solution::count_sub_islands(
            vec![
                vec![1, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 1, 0, 1]
            ],
            vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 0, 0, 1]
            ]
        ),
        2
    );
}
