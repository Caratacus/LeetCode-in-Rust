// Tests for Problem 2742: Painting the Walls
// Java reference: src/test/java/g2701_2800/s2742_painting_the_walls/SolutionTest.java

use leetcode_in_rust::s2742::painting_the_walls::Solution;

#[test]
fn test_paint_walls() {
    assert_eq!(Solution::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]), 3);
}

#[test]
fn test_paint_walls2() {
    assert_eq!(Solution::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]), 4);
}
