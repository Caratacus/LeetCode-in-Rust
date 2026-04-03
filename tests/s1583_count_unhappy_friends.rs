// Tests for Problem 1583: Count Unhappy Friends
// Java reference: src/test/java/g1501_1600/s1583_count_unhappy_friends/SolutionTest.java

use leetcode_in_rust::s1583::count_unhappy_friends::Solution;

#[test]
fn test_unhappy_friends() {
    assert_eq!(
        Solution::unhappy_friends(
            4,
            vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![1, 2, 0]],
            vec![vec![0, 1], vec![2, 3]]
        ),
        2
    );
}

#[test]
fn test_unhappy_friends2() {
    assert_eq!(
        Solution::unhappy_friends(2, vec![vec![1], vec![0]], vec![vec![1, 0]]),
        0
    );
}

#[test]
fn test_unhappy_friends3() {
    assert_eq!(
        Solution::unhappy_friends(
            4,
            vec![vec![1, 3, 2], vec![2, 3, 0], vec![1, 3, 0], vec![0, 2, 1]],
            vec![vec![1, 3], vec![0, 2]]
        ),
        4
    );
}
