// Tests for Problem 2640: Find the Score of All Prefixes of an Array
// Java reference: src/test/java/g2601_2700/s2640_find_the_score_of_all_prefixes_of_an_array/SolutionTest.java

use leetcode_in_rust::s2640::find_the_score_of_all_prefixes_of_an_array::Solution;

#[test]
fn test_find_prefix_score() {
    assert_eq!(
        Solution::find_prefix_score(vec![2, 3, 7, 5, 10]),
        vec![4i64, 10, 24, 36, 56]
    );
}

#[test]
fn test_find_prefix_score2() {
    assert_eq!(
        Solution::find_prefix_score(vec![1, 1, 2, 4, 8, 16]),
        vec![2i64, 4, 8, 16, 32, 64]
    );
}
