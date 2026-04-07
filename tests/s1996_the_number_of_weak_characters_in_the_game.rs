// Tests for Problem 1996: The Number of Weak Characters in the Game
// Java reference: src/test/java/g1901_2000/s1996_the_number_of_weak_characters_in_the_game/SolutionTest.java

use leetcode_in_rust::s1996::the_number_of_weak_characters_in_the_game::Solution;

#[test]
fn test_number_of_weak_characters() {
    assert_eq!(
        Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
        0
    );
}

#[test]
fn test_number_of_weak_characters2() {
    assert_eq!(
        Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
        1
    );
}

#[test]
fn test_number_of_weak_characters3() {
    assert_eq!(
        Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
        1
    );
}
