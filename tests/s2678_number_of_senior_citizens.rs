// Tests for Problem 2678: Number of Senior Citizens
// Java reference: src/test/java/g2601_2700/s2678_number_of_senior_citizens/SolutionTest.java

use leetcode_in_rust::s2678::number_of_senior_citizens::Solution;

#[test]
fn test_count_seniors() {
    assert_eq!(
        Solution::count_seniors(vec![
            "7868190130M7522".to_string(),
            "5303914400F9211".to_string(),
            "9273338290F4010".to_string()
        ]),
        2
    );
}

#[test]
fn test_count_seniors2() {
    assert_eq!(
        Solution::count_seniors(vec![
            "1313579440F2036".to_string(),
            "2921522980M5644".to_string()
        ]),
        0
    );
}
