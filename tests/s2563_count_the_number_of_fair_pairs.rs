// Tests for Problem 2563: Count the Number of Fair Pairs
// Java reference: src/test/java/g2501_2600/s2563_count_the_number_of_fair_pairs/SolutionTest.java

use leetcode_in_rust::s2563::count_the_number_of_fair_pairs::Solution;

#[test]
fn test_count_fair_pairs() {
    assert_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
}

#[test]
fn test_count_fair_pairs2() {
    assert_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
}
