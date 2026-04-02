// Tests for Problem 1189: Maximum Number of Balloons
// Java reference: src/test/java/g1101_1200/s1189_maximum_number_of_balloons/SolutionTest.java

use leetcode_in_rust::s1189::maximum_number_of_balloons::Solution;

#[test]
fn test_max_number_of_balloons() {
    assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
}

#[test]
fn test_max_number_of_balloons2() {
    assert_eq!(
        Solution::max_number_of_balloons("loonbalxballpoon".to_string()),
        2
    );
}

#[test]
fn test_max_number_of_balloons3() {
    assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
}
