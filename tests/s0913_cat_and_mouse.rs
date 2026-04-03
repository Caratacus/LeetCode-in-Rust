// Tests for Problem 0913: Cat and Mouse
// Java reference: src/test/java/g0901_1000/s0913_cat_and_mouse/SolutionTest.java

use leetcode_in_rust::s0913::cat_and_mouse::Solution;

#[test]
fn test_cat_mouse_game() {
    let result = Solution::cat_mouse_game(vec![
        vec![2, 5],
        vec![3],
        vec![0, 4, 5],
        vec![1, 4, 5],
        vec![2, 3],
        vec![0, 2, 3],
    ]);
    assert_eq!(result, 0);
}

#[test]
fn test_cat_mouse_game2() {
    let result = Solution::cat_mouse_game(vec![vec![1, 3], vec![0], vec![3], vec![0, 2]]);
    assert_eq!(result, 1);
}
