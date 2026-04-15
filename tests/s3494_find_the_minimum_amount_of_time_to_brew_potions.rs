// Tests for Problem 3494: Find the Minimum Amount of Time to Brew Potions
// Java reference: src/test/java/g3401_3500/s3494_find_the_minimum_amount_of_time_to_brew_potions/SolutionTest.java

use leetcode_in_rust::s3494::find_the_minimum_amount_of_time_to_brew_potions::Solution;

#[test]
fn test_min_time() {
    assert_eq!(Solution::min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]), 110i64);
}

#[test]
fn test_min_time2() {
    assert_eq!(Solution::min_time(vec![1, 1, 1], vec![1, 1, 1]), 5i64);
}

#[test]
fn test_min_time3() {
    assert_eq!(Solution::min_time(vec![1, 2, 3, 4], vec![1, 2]), 21i64);
}
