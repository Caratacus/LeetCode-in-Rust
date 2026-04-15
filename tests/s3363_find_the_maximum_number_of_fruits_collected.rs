// Tests for Problem 3363: Find the Maximum Number of Fruits Collected
// Java reference: src/test/java/g3301_3400/s3363_find_the_maximum_number_of_fruits_collected/SolutionTest.java

use leetcode_in_rust::s3363::find_the_maximum_number_of_fruits_collected::Solution;

#[test]
fn test_max_collected_fruits() {
    assert_eq!(
        Solution::max_collected_fruits(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 8, 7],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ]),
        100
    );
}

#[test]
fn test_max_collected_fruits2() {
    assert_eq!(
        Solution::max_collected_fruits(vec![vec![1, 1], vec![1, 1]]),
        4
    );
}
