// Tests for Problem 2608: Shortest Cycle in a Graph
// Java reference: src/test/java/g2601_2700/s2608_shortest_cycle_in_a_graph/SolutionTest.java

use leetcode_in_rust::s2608::shortest_cycle_in_a_graph::Solution;

#[test]
fn test_find_shortest_cycle() {
    assert_eq!(
        Solution::find_shortest_cycle(
            7,
            vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 3]]
        ),
        3
    );
}

#[test]
fn test_find_shortest_cycle2() {
    assert_eq!(
        Solution::find_shortest_cycle(4, vec![vec![0, 1], vec![0, 2]]),
        -1
    );
}
