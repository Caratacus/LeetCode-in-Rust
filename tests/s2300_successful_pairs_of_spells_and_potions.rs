// Tests for Problem 2300: Successful Pairs of Spells and Potions
// Java reference: src/test/java/g2201_2300/s2300_successful_pairs_of_spells_and_potions/SolutionTest.java

use leetcode_in_rust::s2300::successful_pairs_of_spells_and_potions::Solution;

#[test]
fn test_successful_pairs() {
    assert_eq!(
        Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
        vec![4, 0, 3]
    );
}

#[test]
fn test_successful_pairs2() {
    assert_eq!(
        Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
        vec![2, 0, 2]
    );
}
