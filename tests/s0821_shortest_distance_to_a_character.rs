// Tests for Problem 0821: Shortest Distance to a Character
// Java reference: src/test/java/g0801_0900/s0821_shortest_distance_to_a_character/SolutionTest.java

use leetcode_in_rust::s0821::shortest_distance_to_a_character::Solution;

#[test]
fn test_shortest_to_char() {
    assert_eq!(
        Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
}

#[test]
fn test_shortest_to_char2() {
    assert_eq!(
        Solution::shortest_to_char("aaab".to_string(), 'b'),
        vec![3, 2, 1, 0]
    );
}
