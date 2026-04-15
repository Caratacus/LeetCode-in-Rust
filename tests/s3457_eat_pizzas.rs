// Tests for Problem 3457: Eat Pizzas
// Java reference: src/test/java/g3401_3500/s3457_eat_pizzas/SolutionTest.java

use leetcode_in_rust::s3457::eat_pizzas::Solution;

#[test]
fn test_max_weight() {
    assert_eq!(Solution::max_weight(vec![1, 2, 3, 4, 5, 6, 7, 8]), 14i64);
}

#[test]
fn test_max_weight2() {
    assert_eq!(Solution::max_weight(vec![2, 1, 1, 1, 1, 1, 1, 1]), 3i64);
}
