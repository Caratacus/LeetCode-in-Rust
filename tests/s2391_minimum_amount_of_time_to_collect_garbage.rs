// Tests for Problem 2391: Minimum Amount of Time to Collect Garbage
// Java reference: src/test/java/g2301_2400/s2391_minimum_amount_of_time_to_collect_garbage/SolutionTest.java

use leetcode_in_rust::s2391::minimum_amount_of_time_to_collect_garbage::Solution;

#[test]
fn test_garbage_collection() {
    assert_eq!(
        Solution::garbage_collection(
            vec!["G".to_string(), "P".to_string(), "GP".to_string(), "GG".to_string()],
            vec![2, 4, 3]
        ),
        21
    );
}

#[test]
fn test_garbage_collection2() {
    assert_eq!(
        Solution::garbage_collection(
            vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()],
            vec![3, 10]
        ),
        37
    );
}
