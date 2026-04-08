// Tests for Problem 3334: Find the Maximum Factor Score of Array
// Java reference: src/test/java/g3301_3400/s3334_find_the_maximum_factor_score_of_array/SolutionTest.java

use leetcode_in_rust::s3334::find_the_maximum_factor_score_of_array::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(vec![2, 4, 8, 16]), 64);
}

#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5]), 60);
}

#[test]
fn test_max_score3() {
    assert_eq!(Solution::max_score(vec![3]), 9);
}
