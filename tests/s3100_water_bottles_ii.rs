// Tests for Problem 3100: Water Bottles II
// Java reference: src/test/java/g3001_3100/s3100_water_bottles_ii/SolutionTest.java

use leetcode_in_rust::s3100::water_bottles_ii::Solution;

#[test]
fn test_max_bottles_drunk() {
    assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
}

#[test]
fn test_max_bottles_drunk2() {
    assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
}
