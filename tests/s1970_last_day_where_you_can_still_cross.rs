// Tests for Problem 1970: Last Day Where You Can Still Cross
// Java reference: src/test/java/g1901_2000/s1970_last_day_where_you_can_still_cross/SolutionTest.java

use leetcode_in_rust::s1970::last_day_where_you_can_still_cross::Solution;

#[test]
fn test_latest_day_to_cross() {
    assert_eq!(
        Solution::latest_day_to_cross(2, 2, vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]]),
        2
    );
}

#[test]
fn test_latest_day_to_cross2() {
    assert_eq!(
        Solution::latest_day_to_cross(2, 2, vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]),
        1
    );
}

#[test]
fn test_latest_day_to_cross3() {
    assert_eq!(
        Solution::latest_day_to_cross(
            3,
            3,
            vec![
                vec![1, 2],
                vec![2, 1],
                vec![3, 3],
                vec![2, 2],
                vec![1, 1],
                vec![1, 3],
                vec![2, 3],
                vec![3, 2],
                vec![3, 1],
            ]
        ),
        3
    );
}
