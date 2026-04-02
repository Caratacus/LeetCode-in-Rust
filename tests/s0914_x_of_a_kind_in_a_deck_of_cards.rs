// Tests for Problem 0914: X of a Kind in a Deck of Cards
// Java reference: src/test/java/g0901_1000/s0914_x_of_a_kind_in_a_deck_of_cards/SolutionTest.java

use leetcode_in_rust::s0914::x_of_a_kind_in_a_deck_of_cards::Solution;

#[test]
fn test_has_groups_size_x() {
    assert_eq!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]), true);
}

#[test]
fn test_has_groups_size_x2() {
    assert_eq!(Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]), false);
}
