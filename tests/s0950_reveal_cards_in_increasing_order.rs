// Tests for Problem 0950: Reveal Cards In Increasing Order
// Java reference: src/test/java/g0901_1000/s0950_reveal_cards_in_increasing_order/SolutionTest.java

use leetcode_in_rust::s0950::reveal_cards_in_increasing_order::Solution;

#[test]
fn test_deck_revealed_increasing() {
    assert_eq!(
        Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
        vec![2, 13, 3, 11, 5, 17, 7]
    );
}

#[test]
fn test_deck_revealed_increasing2() {
    assert_eq!(
        Solution::deck_revealed_increasing(vec![1, 1000]),
        vec![1, 1000]
    );
}
