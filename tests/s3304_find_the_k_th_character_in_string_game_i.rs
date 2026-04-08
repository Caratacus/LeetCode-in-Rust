// Tests for Problem 3304: Find the K-th Character in String Game I
// Java reference: src/test/java/g3301_3400/s3304_find_the_k_th_character_in_string_game_i/SolutionTest.java

use leetcode_in_rust::s3304::find_the_k_th_character_in_string_game_i::Solution;

#[test]
fn test_kth_character() {
    assert_eq!(Solution::kth_character(5), 'b');
}

#[test]
fn test_kth_character2() {
    assert_eq!(Solution::kth_character(10), 'c');
}
