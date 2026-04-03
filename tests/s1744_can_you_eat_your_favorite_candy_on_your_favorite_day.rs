// Tests for Problem 1744: Can You Eat Your Favorite Candy on Your Favorite Day?
// Java reference: src/test/java/g1701_1800/s1744_can_you_eat_your_favorite_candy_on_your_favorite_day/SolutionTest.java

use leetcode_in_rust::s1744::can_you_eat_your_favorite_candy_on_your_favorite_day::Solution;

#[test]
fn test_can_eat() {
    assert_eq!(
        Solution::can_eat(
            vec![7, 4, 5, 3, 8],
            vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1000000000]]
        ),
        vec![true, false, true]
    );
}

#[test]
fn test_can_eat2() {
    assert_eq!(
        Solution::can_eat(
            vec![5, 2, 6, 4, 1],
            vec![vec![3, 1, 2], vec![4, 10, 3], vec![3, 10, 100], vec![4, 100, 30], vec![1, 3, 1]]
        ),
        vec![false, true, true, false, false]
    );
}
