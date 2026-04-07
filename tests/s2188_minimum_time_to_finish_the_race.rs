// Tests for Problem 2188: Minimum Time to Finish the Race
// Java reference: src/test/java/g2101_2200/s2188_minimum_time_to_finish_the_race/SolutionTest.java

use leetcode_in_rust::s2188::minimum_time_to_finish_the_race::Solution;

#[test]
fn test_minimum_finish_time() {
    assert_eq!(
        Solution::minimum_finish_time(vec![vec![2, 3], vec![3, 4]], 5, 4),
        21
    );
}

#[test]
fn test_minimum_finish_time2() {
    assert_eq!(
        Solution::minimum_finish_time(vec![vec![1, 10], vec![2, 2], vec![3, 4]], 6, 5),
        25
    );
}
