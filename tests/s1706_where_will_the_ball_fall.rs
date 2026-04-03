// Tests for Problem 1706: Where Will the Ball Fall
// Java reference: src/test/java/g1701_1800/s1706_where_will_the_ball_fall/SolutionTest.java

use leetcode_in_rust::s1706::where_will_the_ball_fall::Solution;

#[test]
fn test_find_ball() {
    assert_eq!(
        Solution::find_ball(vec![
            vec![1, 1, 1, -1, -1],
            vec![1, 1, 1, -1, -1],
            vec![-1, -1, -1, 1, 1],
            vec![1, 1, 1, 1, -1],
            vec![-1, -1, -1, -1, -1]
        ]),
        vec![1, -1, -1, -1, -1]
    );
}

#[test]
fn test_find_ball2() {
    assert_eq!(Solution::find_ball(vec![vec![-1]]), vec![-1]);
}

#[test]
fn test_find_ball3() {
    assert_eq!(
        Solution::find_ball(vec![
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1]
        ]),
        vec![0, 1, 2, 3, 4, -1]
    );
}
