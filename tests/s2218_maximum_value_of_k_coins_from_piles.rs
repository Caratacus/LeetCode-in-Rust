// Tests for Problem 2218: Maximum Value of K Coins From Piles
// Java reference: src/test/java/g2201_2300/s2218_maximum_value_of_k_coins_from_piles/SolutionTest.java

use leetcode_in_rust::s2218::maximum_value_of_k_coins_from_piles::Solution;

#[test]
fn test_max_value_of_coins() {
    assert_eq!(
        Solution::max_value_of_coins(
            vec![vec![1, 100, 3], vec![7, 8, 9]],
            2
        ),
        101
    );
}

#[test]
fn test_max_value_of_coins2() {
    assert_eq!(
        Solution::max_value_of_coins(
            vec![
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![1, 1, 1, 1, 1, 1, 700]
            ],
            7
        ),
        706
    );
}
