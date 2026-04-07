// Tests for Problem 2201: Count Artifacts That Can Be Extracted
// Java reference: src/test/java/g2201_2300/s2201_count_artifacts_that_can_be_extracted/SolutionTest.java

use leetcode_in_rust::s2201::count_artifacts_that_can_be_extracted::Solution;

#[test]
fn test_dig_artifacts() {
    assert_eq!(
        Solution::dig_artifacts(
            2,
            vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
            vec![vec![0, 0], vec![0, 1]]
        ),
        1
    );
}

#[test]
fn test_dig_artifacts2() {
    assert_eq!(
        Solution::dig_artifacts(
            2,
            vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
            vec![vec![0, 0], vec![0, 1], vec![1, 1]]
        ),
        2
    );
}
