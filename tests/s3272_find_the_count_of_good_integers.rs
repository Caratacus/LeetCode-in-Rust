// Tests for Problem 3272: Find the Count of Good Integers
// Java reference: src/test/java/g3201_3300/s3272_find_the_count_of_good_integers/SolutionTest.java

use leetcode_in_rust::s3272::find_the_count_of_good_integers::Solution;

#[test]
fn test_count_good_integers() {
    assert_eq!(Solution::count_good_integers(3, 5), 27);
}

#[test]
fn test_count_good_integers2() {
    assert_eq!(Solution::count_good_integers(1, 4), 2);
}

#[test]
fn test_count_good_integers3() {
    assert_eq!(Solution::count_good_integers(5, 6), 2468);
}
