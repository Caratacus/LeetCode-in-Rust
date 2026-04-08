// Tests for Problem 3226: Number of Bit Changes to Make Two Integers Equal
// Java reference: src/test/java/g3201_3300/s3226_number_of_bit_changes_to_make_two_integers_equal/SolutionTest.java

use leetcode_in_rust::s3226::number_of_bit_changes_to_make_two_integers_equal::Solution;

#[test]
fn test_min_changes() {
    assert_eq!(Solution::min_changes(13, 4), 2);
}

#[test]
fn test_min_changes2() {
    assert_eq!(Solution::min_changes(21, 21), 0);
}

#[test]
fn test_min_changes3() {
    assert_eq!(Solution::min_changes(14, 13), -1);
}
