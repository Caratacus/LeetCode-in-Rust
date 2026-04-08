// Tests for Problem 3307: Find the K-th Character in String Game II
// Java reference: src/test/java/g3301_3400/s3307_find_the_k_th_character_in_string_game_ii/SolutionTest.java

use leetcode_in_rust::s3307::find_the_k_th_character_in_string_game_ii::Solution;

#[test]
fn test_kth_character() {
    assert_eq!(Solution::kth_character(5, vec![0, 0, 0]), 'a');
}

#[test]
fn test_kth_character2() {
    assert_eq!(Solution::kth_character(10, vec![0, 1, 0, 1]), 'b');
}
