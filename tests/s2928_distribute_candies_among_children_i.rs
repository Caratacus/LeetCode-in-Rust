// Tests for Problem 2928: Distribute Candies Among Children I
// Java reference: src/test/java/g2901_3000/s2928_distribute_candies_among_children_i/SolutionTest.java

use leetcode_in_rust::s2928::distribute_candies_among_children_i::Solution;

#[test]
fn test_distribute_candies() {
    assert_eq!(Solution::distribute_candies(5, 2), 3);
}

#[test]
fn test_distribute_candies2() {
    assert_eq!(Solution::distribute_candies(3, 3), 10);
}
