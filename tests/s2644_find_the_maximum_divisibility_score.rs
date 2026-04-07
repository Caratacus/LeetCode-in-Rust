// Tests for Problem 2644: Find the Maximum Divisibility Score
// Java reference: src/test/java/g2601_2700/s2644_find_the_maximum_divisibility_score/SolutionTest.java

use leetcode_in_rust::s2644::find_the_maximum_divisibility_score::Solution;

#[test]
fn test_max_div_score() {
    assert_eq!(
        Solution::max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3]),
        3
    );
}

#[test]
fn test_max_div_score2() {
    assert_eq!(
        Solution::max_div_score(vec![20, 14, 21, 10], vec![5, 7, 5]),
        5
    );
}

#[test]
fn test_max_div_score3() {
    assert_eq!(Solution::max_div_score(vec![12], vec![10]), 10);
}
