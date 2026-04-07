// Tests for Problem 2530: Maximal Score After Applying K operations
// Java reference: src/test/java/g2501_2600/s2530_maximal_score_after_applying_k_operations/SolutionTest.java

use leetcode_in_rust::s2530::maximal_score_after_applying_k_operations::Solution;

#[test]
fn test_max_kelements() {
    assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
}

#[test]
fn test_max_kelements2() {
    assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
}
