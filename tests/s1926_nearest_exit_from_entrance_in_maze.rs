// Tests for Problem 1926: Nearest Exit from Entrance in Maze
// Java reference: src/test/java/g1901_2000/s1926_nearest_exit_from_entrance_in_maze/SolutionTest.java

use leetcode_in_rust::s1926::nearest_exit_from_entrance_in_maze::Solution;

#[test]
fn test_nearest_exit() {
    assert_eq!(
        Solution::nearest_exit(
            vec![
                vec!['+', '+', '.', '+'],
                vec!['.', '.', '.', '+'],
                vec!['+', '+', '+', '.']
            ],
            vec![1, 2]
        ),
        1
    );
}

#[test]
fn test_nearest_exit2() {
    assert_eq!(
        Solution::nearest_exit(
            vec![
                vec!['+', '+', '+'],
                vec!['.', '.', '.'],
                vec!['+', '+', '+']
            ],
            vec![1, 0]
        ),
        2
    );
}

#[test]
fn test_nearest_exit3() {
    assert_eq!(
        Solution::nearest_exit(vec![vec!['.', '+']], vec![0, 0]),
        -1
    );
}
