// Tests for Problem 2940: Find Building Where Alice and Bob Can Meet
// Java reference: src/test/java/g2901_3000/s2940_find_building_where_alice_and_bob_can_meet/SolutionTest.java

use leetcode_in_rust::s2940::find_building_where_alice_and_bob_can_meet::Solution;

#[test]
fn test_leftmost_building_queries() {
    assert_eq!(
        Solution::leftmost_building_queries(
            vec![6, 4, 8, 5, 2, 7],
            vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]
        ),
        vec![2, 5, -1, 5, 2]
    );
}
