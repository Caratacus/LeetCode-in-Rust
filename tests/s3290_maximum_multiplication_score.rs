// Tests for Problem 3290: Maximum Multiplication Score
// Java reference: src/test/java/g3201_3300/s3290_maximum_multiplication_score/SolutionTest.java

use leetcode_in_rust::s3290::maximum_multiplication_score::Solution;

#[test]
fn test_max_score() {
    assert_eq!(
        Solution::max_score(vec![3, 2, 5, 6], vec![2, -6, 4, -5, -3, 2, -7]),
        26
    );
}

#[test]
fn test_max_score2() {
    assert_eq!(
        Solution::max_score(vec![-1, 4, 5, -2], vec![-5, -1, -3, -2, -4]),
        -1
    );
}
