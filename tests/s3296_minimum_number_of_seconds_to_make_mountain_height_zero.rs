// Tests for Problem 3296: Minimum Number of Seconds to Make Mountain Height Zero
// Java reference: src/test/java/g3201_3300/s3296_minimum_number_of_seconds_to_make_mountain_height_zero/SolutionTest.java

use leetcode_in_rust::s3296::minimum_number_of_seconds_to_make_mountain_height_zero::Solution;

#[test]
fn test_min_number_of_seconds() {
    assert_eq!(Solution::min_number_of_seconds(4, vec![2, 1, 1]), 3);
}

#[test]
fn test_min_number_of_seconds2() {
    assert_eq!(Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]), 12);
}

#[test]
fn test_min_number_of_seconds3() {
    assert_eq!(Solution::min_number_of_seconds(5, vec![1]), 15);
}
