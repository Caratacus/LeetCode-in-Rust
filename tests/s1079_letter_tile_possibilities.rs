// Tests for Problem 1079: Letter Tile Possibilities
// Java reference: src/test/java/g1001_1100/s1079_letter_tile_possibilities/SolutionTest.java

use leetcode_in_rust::s1079::letter_tile_possibilities::Solution;

#[test]
fn test_num_tile_possibilities() {
    assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
}

#[test]
fn test_num_tile_possibilities2() {
    assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
}

#[test]
fn test_num_tile_possibilities3() {
    assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
}
