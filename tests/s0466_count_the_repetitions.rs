// Tests for Problem 0466: Count the Repetitions
// Java reference: src/test/java/g0401_0500/s0466_count_the_repetitions/SolutionTest.java

use leetcode_in_rust::s0466::count_the_repetitions::Solution;

#[test]
fn test_get_max_repetitions() {
    assert_eq!(Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2), 2);
}

#[test]
fn test_get_max_repetitions2() {
    assert_eq!(Solution::get_max_repetitions("acb".to_string(), 1, "acb".to_string(), 1), 1);
}
