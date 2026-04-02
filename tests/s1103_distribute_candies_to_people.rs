// Tests for Problem 1103: Distribute Candies to People
// Java reference: src/test/java/g1101_1200/s1103_distribute_candies_to_people/SolutionTest.java

use leetcode_in_rust::s1103::distribute_candies_to_people::Solution;

#[test]
fn test_distribute_candies() {
    assert_eq!(Solution::distribute_candies(7, 4), vec![1, 2, 3, 1]);
}

#[test]
fn test_distribute_candies2() {
    assert_eq!(Solution::distribute_candies(10, 3), vec![5, 2, 3]);
}
