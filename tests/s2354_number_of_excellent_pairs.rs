// Tests for Problem 2354: Number of Excellent Pairs
// Java reference: src/test/java/g2301_2400/s2354_number_of_excellent_pairs/SolutionTest.java

use leetcode_in_rust::s2354::number_of_excellent_pairs::Solution;

#[test]
fn test_count_excellent_pairs() {
    assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 3, 1], 3), 5);
}

#[test]
fn test_count_excellent_pairs2() {
    assert_eq!(Solution::count_excellent_pairs(vec![5, 1, 1], 10), 0);
}
