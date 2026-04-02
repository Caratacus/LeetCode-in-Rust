// Tests for Problem 1090: Largest Values From Labels
// Java reference: src/test/java/g1001_1100/s1090_largest_values_from_labels/SolutionTest.java

use leetcode_in_rust::s1090::largest_values_from_labels::Solution;

#[test]
fn test_largest_vals_from_labels() {
    assert_eq!(
        Solution::largest_vals_from_labels(
            vec![5, 4, 3, 2, 1],
            vec![1, 1, 2, 2, 3],
            3,
            1
        ),
        9
    );
}

#[test]
fn test_largest_vals_from_labels2() {
    assert_eq!(
        Solution::largest_vals_from_labels(
            vec![5, 4, 3, 2, 1],
            vec![1, 3, 3, 3, 2],
            3,
            2
        ),
        12
    );
}
