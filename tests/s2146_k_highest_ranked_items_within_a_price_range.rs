// Tests for Problem 2146: K Highest Ranked Items Within a Price Range
// Java reference: src/test/java/g2101_2200/s2146_k_highest_ranked_items_within_a_price_range/SolutionTest.java

use leetcode_in_rust::s2146::k_highest_ranked_items_within_a_price_range::Solution;

#[test]
fn test_highest_ranked_k_items() {
    assert_eq!(
        Solution::highest_ranked_k_items(
            vec![vec![1, 2, 0, 1], vec![1, 3, 0, 1], vec![0, 2, 5, 1]],
            vec![2, 5],
            vec![0, 0],
            3
        ),
        vec![vec![2, 2], vec![1, 1], vec![0, 1]]
    );
}

#[test]
fn test_highest_ranked_k_items2() {
    assert_eq!(
        Solution::highest_ranked_k_items(
            vec![vec![1, 2, 0, 1], vec![1, 3, 3, 1], vec![0, 2, 5, 1]],
            vec![2, 3],
            vec![0, 0],
            2
        ),
        vec![vec![0, 1], vec![1, 1]]
    );
}

#[test]
fn test_highest_ranked_k_items3() {
    assert_eq!(
        Solution::highest_ranked_k_items(
            vec![vec![1, 1, 1], vec![0, 0, 1], vec![2, 3, 4]],
            vec![2, 3],
            vec![0, 0],
            3
        ),
        vec![vec![2, 1], vec![2, 0]]
    );
}
