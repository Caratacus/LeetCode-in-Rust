// Tests for Problem 1263: Minimum Moves to Move a Box to Their Target Location
// Java reference: src/test/java/g1201_1300/s1263_minimum_moves_to_move_a_box_to_their_target_location/SolutionTest.java

use leetcode_in_rust::s1263::minimum_moves_to_move_a_box_to_their_target_location::Solution;

#[test]
fn test_min_push_box() {
    let result = Solution::min_push_box(vec![
        vec!['#', '#', '#', '#', '#', '#'],
        vec!['#', 'T', '#', '#', '#', '#'],
        vec!['#', '.', '.', 'B', '.', '#'],
        vec!['#', '.', '#', '#', '.', '#'],
        vec!['#', '.', '.', '.', 'S', '#'],
        vec!['#', '#', '#', '#', '#', '#'],
    ]);
    assert_eq!(result, 3);
}

#[test]
fn test_min_push_box2() {
    let result = Solution::min_push_box(vec![
        vec!['#', '#', '#', '#', '#', '#'],
        vec!['#', 'T', '#', '#', '#', '#'],
        vec!['#', '.', '.', 'B', '.', '#'],
        vec!['#', '#', '#', '#', '.', '#'],
        vec!['#', '.', '.', '.', 'S', '#'],
        vec!['#', '#', '#', '#', '#', '#'],
    ]);
    assert_eq!(result, -1);
}

#[test]
fn test_min_push_box3() {
    let result = Solution::min_push_box(vec![
        vec!['#', '#', '#', '#', '#', '#'],
        vec!['#', 'T', '.', '.', '#', '#'],
        vec!['#', '.', '#', 'B', '.', '#'],
        vec!['#', '.', '.', '.', '.', '#'],
        vec!['#', '.', '.', '.', 'S', '#'],
        vec!['#', '#', '#', '#', '#', '#'],
    ]);
    assert_eq!(result, 5);
}
