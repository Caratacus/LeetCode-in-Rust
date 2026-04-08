// Tests for Problem 3320: Count the Number of Winning Sequences
// Java reference: src/test/java/g3301_3400/s3320_count_the_number_of_winning_sequences/SolutionTest.java

use leetcode_in_rust::s3320::count_the_number_of_winning_sequences::Solution;

#[test]
fn test_count_winning_sequences() {
    assert_eq!(Solution::count_winning_sequences("FFF".to_string()), 3);
}

#[test]
fn test_count_winning_sequences2() {
    assert_eq!(Solution::count_winning_sequences("FWEFW".to_string()), 18);
}
