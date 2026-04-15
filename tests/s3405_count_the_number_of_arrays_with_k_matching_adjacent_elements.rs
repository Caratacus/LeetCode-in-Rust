// Tests for Problem 3405: Count the Number of Arrays With K Matching Adjacent Elements
// Java reference: src/test/java/g3401_3500/s3405_count_the_number_of_arrays_with_k_matching_adjacent_elements/SolutionTest.java

use leetcode_in_rust::s3405::count_the_number_of_arrays_with_k_matching_adjacent_elements::Solution;

#[test]
fn test_count_good_arrays() {
    assert_eq!(Solution::count_good_arrays(3, 2, 1), 4);
}

#[test]
fn test_count_good_arrays2() {
    assert_eq!(Solution::count_good_arrays(4, 2, 2), 6);
}

#[test]
fn test_count_good_arrays3() {
    assert_eq!(Solution::count_good_arrays(5, 2, 0), 2);
}
