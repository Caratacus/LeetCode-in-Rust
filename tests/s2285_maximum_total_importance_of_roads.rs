// Tests for Problem 2285: Maximum Total Importance of Roads
// Java reference: src/test/java/g2201_2300/s2285_maximum_total_importance_of_roads/SolutionTest.java

use leetcode_in_rust::s2285::maximum_total_importance_of_roads::Solution;

#[test]
fn test_maximum_importance() {
    assert_eq!(
        Solution::maximum_importance(
            5,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![0, 2],
                vec![1, 3],
                vec![2, 4]
            ]
        ),
        43
    );
}

#[test]
fn test_maximum_importance2() {
    assert_eq!(
        Solution::maximum_importance(5, vec![vec![0, 3], vec![2, 4], vec![1, 3]]),
        20
    );
}
