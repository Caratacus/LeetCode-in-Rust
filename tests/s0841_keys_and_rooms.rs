// Tests for Problem 0841: Keys and Rooms
// Java reference: src/test/java/g0801_0900/s0841_keys_and_rooms/SolutionTest.java

use leetcode_in_rust::s0841::keys_and_rooms::Solution;

#[test]
fn test_can_visit_all_rooms() {
    assert_eq!(
        Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3]]),
        true
    );
}

#[test]
fn test_can_visit_all_rooms2() {
    assert_eq!(
        Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0], vec![2]]),
        false
    );
}
