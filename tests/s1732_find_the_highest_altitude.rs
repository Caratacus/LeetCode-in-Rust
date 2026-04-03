// Tests for Problem 1732: Find the Highest Altitude
// Java reference: src/test/java/g1701_1800/s1732_find_the_highest_altitude/SolutionTest.java

use leetcode_in_rust::s1732::find_the_highest_altitude::Solution;

#[test]
fn test_largest_altitude() {
    assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
}

#[test]
fn test_largest_altitude2() {
    assert_eq!(
        Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]),
        0
    );
}
