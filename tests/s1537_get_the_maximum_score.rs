// Tests for Problem 1537: Get the Maximum Score
// Java reference: src/test/java/g1501_1600/s1537_get_the_maximum_score/SolutionTest.java

use leetcode_in_rust::s1537::get_the_maximum_score::Solution;

#[test]
fn test_max_sum() {
    assert_eq!(Solution::max_sum(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9]), 30);
}

#[test]
fn test_max_sum2() {
    assert_eq!(Solution::max_sum(vec![1, 3, 5, 7, 9], vec![3, 5, 100]), 109);
}

#[test]
fn test_max_sum3() {
    assert_eq!(Solution::max_sum(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]), 40);
}
