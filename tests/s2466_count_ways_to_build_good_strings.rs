// Tests for Problem 2466: Count Ways to Build Good Strings
// Java reference: src/test/java/g2401_2500/s2466_count_ways_to_build_good_strings/SolutionTest.java

use leetcode_in_rust::s2466::count_ways_to_build_good_strings::Solution;

#[test]
fn test_count_good_strings() {
    assert_eq!(Solution::count_good_strings(3, 3, 1, 1), 8);
}

#[test]
fn test_count_good_strings2() {
    assert_eq!(Solution::count_good_strings(2, 3, 1, 2), 5);
}
