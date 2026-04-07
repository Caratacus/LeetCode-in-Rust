// Tests for Problem 1854: Maximum Population Year
// Java reference: src/test/java/g1801_1900/s1854_maximum_population_year/SolutionTest.java

use leetcode_in_rust::s1854::maximum_population_year::Solution;

#[test]
fn test_maximum_population() {
    assert_eq!(
        Solution::maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]),
        1993
    );
}

#[test]
fn test_maximum_population2() {
    assert_eq!(
        Solution::maximum_population(vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]]),
        1960
    );
}
