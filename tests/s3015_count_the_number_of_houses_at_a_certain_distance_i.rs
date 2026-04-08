// Tests for Problem 3015: Count the Number of Houses at a Certain Distance I
// Java reference: src/test/java/g3001_3100/s3015_count_the_number_of_houses_at_a_certain_distance_i/SolutionTest.java

use leetcode_in_rust::s3015::count_the_number_of_houses_at_a_certain_distance_i::Solution;

#[test]
fn test_count_of_pairs() {
    assert_eq!(Solution::count_of_pairs(3, 1, 3), vec![6, 0, 0]);
}

#[test]
fn test_count_of_pairs2() {
    assert_eq!(Solution::count_of_pairs(5, 2, 4), vec![10, 8, 2, 0, 0]);
}

#[test]
fn test_count_of_pairs3() {
    assert_eq!(Solution::count_of_pairs(4, 1, 1), vec![6, 4, 2, 0]);
}
