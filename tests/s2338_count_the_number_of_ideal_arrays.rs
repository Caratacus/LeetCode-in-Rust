// Tests for Problem 2338: Count The Number of Ideal Arrays
// Java reference: src/test/java/g2301_2400/s2338_count_the_number_of_ideal_arrays/SolutionTest.java

use leetcode_in_rust::s2338::count_the_number_of_ideal_arrays::Solution;

#[test]
fn test_ideal_arrays() {
    assert_eq!(Solution::ideal_arrays(2, 5), 10);
}

#[test]
fn test_ideal_arrays2() {
    assert_eq!(Solution::ideal_arrays(5, 3), 11);
}
