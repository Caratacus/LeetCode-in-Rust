// Tests for Problem 2929: Distribute Candies Among Children II
// Java reference: src/test/java/g2901_3000/s2929_distribute_candies_among_children_ii/SolutionTest.java

use leetcode_in_rust::s2929::distribute_candies_among_children_ii::Solution;

#[test]
fn test_distribute_candies() {
    assert_eq!(Solution::distribute_candies(5, 2), 3);
}

#[test]
fn test_distribute_candies2() {
    assert_eq!(Solution::distribute_candies(3, 3), 10);
}
