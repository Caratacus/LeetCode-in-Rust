// Tests for Problem 1481: Least Number of Unique Integers after K Removals
// Java reference: src/test/java/g1401_1500/s1481_least_number_of_unique_integers_after_k_removals/SolutionTest.java

use leetcode_in_rust::s1481::least_number_of_unique_integers_after_k_removals::Solution;

#[test]
fn test_find_least_num_of_unique_ints() {
    assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
}

#[test]
fn test_find_least_num_of_unique_ints2() {
    assert_eq!(Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3), 2);
}
