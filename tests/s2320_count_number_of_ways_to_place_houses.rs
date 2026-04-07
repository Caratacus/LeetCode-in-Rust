// Tests for Problem 2320: Count Number of Ways to Place Houses
// Java reference: src/test/java/g2301_2400/s2320_count_number_of_ways_to_place_houses/SolutionTest.java

use leetcode_in_rust::s2320::count_number_of_ways_to_place_houses::Solution;

#[test]
fn test_count_house_placements() {
    assert_eq!(Solution::count_house_placements(1), 4);
}

#[test]
fn test_count_house_placements2() {
    assert_eq!(Solution::count_house_placements(2), 9);
}
