// Tests for Problem 2222: Number of Ways to Select Buildings
// Java reference: src/test/java/g2201_2300/s2222_number_of_ways_to_select_buildings/SolutionTest.java

use leetcode_in_rust::s2222::number_of_ways_to_select_buildings::Solution;

#[test]
fn test_number_of_ways() {
    assert_eq!(Solution::number_of_ways("001101".to_string()), 6);
}

#[test]
fn test_number_of_ways2() {
    assert_eq!(Solution::number_of_ways("11100".to_string()), 0);
}
