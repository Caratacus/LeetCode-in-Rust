// Tests for Problem 2106: Maximum Fruits Harvested After at Most K Steps
// Java reference: src/test/java/g2101_2200/s2106_maximum_fruits_harvested_after_at_most_k_steps/SolutionTest.java

use leetcode_in_rust::s2106::maximum_fruits_harvested_after_at_most_k_steps::Solution;

#[test]
fn test_max_total_fruits() {
    assert_eq!(
        Solution::max_total_fruits(vec![vec![2, 8], vec![6, 3], vec![8, 6]], 5, 4),
        9
    );
}

#[test]
fn test_max_total_fruits2() {
    assert_eq!(
        Solution::max_total_fruits(
            vec![vec![0, 9], vec![4, 1], vec![5, 7], vec![6, 2], vec![7, 4], vec![10, 9]],
            5,
            4
        ),
        14
    );
}

#[test]
fn test_max_total_fruits3() {
    assert_eq!(
        Solution::max_total_fruits(vec![vec![0, 3], vec![6, 4], vec![8, 5]], 3, 2),
        0
    );
}
