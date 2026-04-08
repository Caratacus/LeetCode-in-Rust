// Tests for Problem 3194: Minimum Average of Smallest and Largest Elements
// Java reference: src/test/java/g3101_3200/s3194_minimum_average_of_smallest_and_largest_elements/SolutionTest.java

use leetcode_in_rust::s3194::minimum_average_of_smallest_and_largest_elements::Solution;

#[test]
fn test_minimum_average() {
    let result = Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]);
    assert!((result - 5.5).abs() < 1e-9);
}

#[test]
fn test_minimum_average2() {
    let result = Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]);
    assert!((result - 5.5).abs() < 1e-9);
}

#[test]
fn test_minimum_average3() {
    let result = Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]);
    assert!((result - 5.0).abs() < 1e-9);
}
