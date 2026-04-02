// Tests for Problem 1222: Queens That Can Attack the King
// Java reference: src/test/java/g1201_1300/s1222_queens_that_can_attack_the_king/SolutionTest.java

use leetcode_in_rust::s1222::queens_that_can_attack_the_king::Solution;

#[test]
fn test_queens_attackthe_king() {
    let mut result = Solution::queens_attackthe_king(
        vec![vec![0, 1], vec![1, 0], vec![4, 0], vec![0, 4], vec![3, 3], vec![2, 4]],
        vec![0, 0],
    );
    for pos in &mut result {
        pos.sort();
    }
    result.sort();
    assert_eq!(result, vec![vec![0, 1], vec![1, 0], vec![3, 3]]);
}

#[test]
fn test_queens_attackthe_king2() {
    let mut result = Solution::queens_attackthe_king(
        vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 4], vec![3, 5], vec![4, 4], vec![4, 5]],
        vec![3, 3],
    );
    for pos in &mut result {
        pos.sort();
    }
    result.sort();
    assert_eq!(result, vec![vec![2, 2], vec![3, 4], vec![4, 4]]);
}
