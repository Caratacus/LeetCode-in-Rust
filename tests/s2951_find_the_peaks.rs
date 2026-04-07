// Tests for Problem 2951: Find the Peaks
// Java reference: src/test/java/g2901_3000/s2951_find_the_peaks/SolutionTest.java

use leetcode_in_rust::s2951::find_the_peaks::Solution;

#[test]
fn test_find_peaks() {
    assert_eq!(Solution::find_peaks(vec![2, 4, 4]), vec![] as Vec<i32>);
}

#[test]
fn test_find_peaks2() {
    assert_eq!(Solution::find_peaks(vec![1, 4, 3, 8, 5]), vec![1, 3]);
}
