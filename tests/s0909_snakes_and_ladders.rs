// Tests for Problem 0909: Snakes and Ladders
// Java reference: src/test/java/g0901_1000/s0909_snakes_and_ladders/SolutionTest.java

use leetcode_in_rust::s0909::snakes_and_ladders::Solution;

#[test]
fn test_snakes_and_ladders() {
    assert_eq!(
        Solution::snakes_and_ladders(vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1]
        ]),
        4
    );
}

#[test]
fn test_snakes_and_ladders2() {
    assert_eq!(
        Solution::snakes_and_ladders(vec![vec![-1, -1], vec![-1, 3]]),
        1
    );
}
