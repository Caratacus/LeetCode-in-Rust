// Tests for Problem 2359: Find Closest Node to Given Two Nodes
// Java reference: src/test/java/g2301_2400/s2359_find_closest_node_to_given_two_nodes/SolutionTest.java

use leetcode_in_rust::s2359::find_closest_node_to_given_two_nodes::Solution;

#[test]
fn test_closest_meeting_node() {
    assert_eq!(Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1), 2);
}

#[test]
fn test_closest_meeting_node2() {
    assert_eq!(Solution::closest_meeting_node(vec![1, 2, -1], 0, 2), 2);
}
