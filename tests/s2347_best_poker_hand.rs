// Tests for Problem 2347: Best Poker Hand
// Java reference: src/test/java/g2301_2400/s2347_best_poker_hand/SolutionTest.java

use leetcode_in_rust::s2347::best_poker_hand::Solution;

#[test]
fn test_best_hand() {
    assert_eq!(
        Solution::best_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']),
        "Flush".to_string()
    );
}

#[test]
fn test_best_hand2() {
    assert_eq!(
        Solution::best_hand(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']),
        "Three of a Kind".to_string()
    );
}

#[test]
fn test_best_hand3() {
    assert_eq!(
        Solution::best_hand(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
        "Pair".to_string()
    );
}

#[test]
fn test_best_hand4() {
    assert_eq!(
        Solution::best_hand(vec![13, 12, 3, 4, 7], vec!['a', 'd', 'c', 'b', 'c']),
        "High Card".to_string()
    );
}
