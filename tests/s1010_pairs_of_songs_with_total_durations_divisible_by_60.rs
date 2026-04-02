// Tests for Problem 1010: Pairs of Songs With Total Durations Divisible by 60
// Java reference: src/test/java/g1001_1100/s1010_pairs_of_songs_with_total_durations_divisible_by_60/SolutionTest.java

use leetcode_in_rust::s1010::pairs_of_songs_with_total_durations_divisible_by_60::Solution;

#[test]
fn test_num_pairs_divisible_by60() {
    assert_eq!(Solution::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]), 3);
}

#[test]
fn test_num_pairs_divisible_by60_2() {
    assert_eq!(Solution::num_pairs_divisible_by60(vec![60, 60, 60]), 3);
}
