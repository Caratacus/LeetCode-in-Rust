// Tests for Problem 1219: Path with Maximum Gold
// Java reference: src/test/java/g1201_1300/s1219_path_with_maximum_gold/SolutionTest.java

use leetcode_in_rust::s1219::path_with_maximum_gold::Solution;

#[test]
fn test_get_maximum_gold() {
    assert_eq!(
        Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
        24
    );
}

#[test]
fn test_get_maximum_gold2() {
    assert_eq!(
        Solution::get_maximum_gold(vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20]
        ]),
        28
    );
}
