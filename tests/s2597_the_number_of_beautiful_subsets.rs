// Tests for Problem 2597: The Number of Beautiful Subsets
// Java reference: src/test/java/g2501_2600/s2597_the_number_of_beautiful_subsets/SolutionTest.java

use leetcode_in_rust::s2597::the_number_of_beautiful_subsets::Solution;

#[test]
fn test_beautiful_subsets() {
    assert_eq!(Solution::beautiful_subsets(vec![2, 4, 6], 2), 4);
}

#[test]
fn test_beautiful_subsets2() {
    assert_eq!(Solution::beautiful_subsets(vec![1], 1), 1);
}
