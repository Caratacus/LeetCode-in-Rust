// Tests for Problem 2593: Find Score of an Array After Marking All Elements
// Java reference: src/test/java/g2501_2600/s2593_find_score_of_an_array_after_marking_all_elements/SolutionTest.java

use leetcode_in_rust::s2593::find_score_of_an_array_after_marking_all_elements::Solution;

#[test]
fn test_find_score() {
    assert_eq!(Solution::find_score(vec![2, 1, 3, 4, 5, 2]), 7);
}

#[test]
fn test_find_score2() {
    assert_eq!(Solution::find_score(vec![2, 3, 5, 1, 3, 2]), 5);
}
