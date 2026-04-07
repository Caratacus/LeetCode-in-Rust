// Tests for Problem 2209: Minimum White Tiles After Covering With Carpets
// Java reference: src/test/java/g2201_2300/s2209_minimum_white_tiles_after_covering_with_carpets/SolutionTest.java

use leetcode_in_rust::s2209::minimum_white_tiles_after_covering_with_carpets::Solution;

#[test]
fn test_minimum_white_tiles() {
    assert_eq!(Solution::minimum_white_tiles("10110101".to_string(), 2, 2), 2);
}

#[test]
fn test_minimum_white_tiles2() {
    assert_eq!(Solution::minimum_white_tiles("11111".to_string(), 2, 3), 0);
}
