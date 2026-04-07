// Tests for Problem 1916: Count Ways to Build Rooms in an Ant Colony
// Java reference: src/test/java/g1901_2000/s1916_count_ways_to_build_rooms_in_an_ant_colony/SolutionTest.java

use leetcode_in_rust::s1916::count_ways_to_build_rooms_in_an_ant_colony::Solution;

#[test]
fn test_ways_to_build_rooms() {
    assert_eq!(Solution::ways_to_build_rooms(vec![-1, 0, 1]), 1);
}

#[test]
fn test_ways_to_build_rooms2() {
    assert_eq!(Solution::ways_to_build_rooms(vec![-1, 0, 0, 1, 2]), 6);
}
