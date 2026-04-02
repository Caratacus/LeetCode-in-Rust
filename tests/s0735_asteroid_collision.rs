// Tests for Problem 0735: Asteroid Collision
// Java reference: src/test/java/g0701_0800/s0735_asteroid_collision/SolutionTest.java

use leetcode_in_rust::s0735::asteroid_collision::Solution;

#[test]
fn test_asteroid_collision() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
}

#[test]
fn test_asteroid_collision2() {
    assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
}

#[test]
fn test_asteroid_collision3() {
    assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
}
