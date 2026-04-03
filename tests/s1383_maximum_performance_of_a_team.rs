// Tests for Problem 1383: Maximum Performance of a Team
// Java reference: src/test/java/g1301_1400/s1383_maximum_performance_of_a_team/SolutionTest.java

use leetcode_in_rust::s1383::maximum_performance_of_a_team::Solution;

#[test]
fn test_max_performance() {
    // n=6, speed=[2, 10, 3, 1, 5, 8], efficiency=[5, 4, 3, 9, 7, 2], k=2
    assert_eq!(
        Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
        60
    );
}

#[test]
fn test_max_performance2() {
    // n=6, speed=[2, 10, 3, 1, 5, 8], efficiency=[5, 4, 3, 9, 7, 2], k=3
    assert_eq!(
        Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
        68
    );
}

#[test]
fn test_max_performance3() {
    // n=6, speed=[2, 10, 3, 1, 5, 8], efficiency=[5, 4, 3, 9, 7, 2], k=4
    assert_eq!(
        Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
        72
    );
}
