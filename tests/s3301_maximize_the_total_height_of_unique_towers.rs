// Tests for Problem 3301: Maximize the Total Height of Unique Towers
// Java reference: src/test/java/g3301_3400/s3301_maximize_the_total_height_of_unique_towers/SolutionTest.java

use leetcode_in_rust::s3301::maximize_the_total_height_of_unique_towers::Solution;

#[test]
fn test_maximum_total_sum() {
    assert_eq!(Solution::maximum_total_sum(vec![2, 3, 4, 3]), 10);
}

#[test]
fn test_maximum_total_sum2() {
    assert_eq!(Solution::maximum_total_sum(vec![15, 10]), 25);
}

#[test]
fn test_maximum_total_sum3() {
    assert_eq!(Solution::maximum_total_sum(vec![2, 2, 1]), -1);
}
