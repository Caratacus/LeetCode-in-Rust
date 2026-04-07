// Tests for Problem 2833: Furthest Point From Origin
// Java reference: src/test/java/g2801_2900/s2833_furthest_point_from_origin/SolutionTest.java

use leetcode_in_rust::s2833::furthest_point_from_origin::Solution;

#[test]
fn test_furthest_distance_from_origin() {
    assert_eq!(Solution::furthest_distance_from_origin("L_RL__R".to_string()), 3);
}

#[test]
fn test_furthest_distance_from_origin2() {
    assert_eq!(Solution::furthest_distance_from_origin("_R__LL_".to_string()), 5);
}

#[test]
fn test_furthest_distance_from_origin3() {
    assert_eq!(Solution::furthest_distance_from_origin("_______".to_string()), 7);
}
