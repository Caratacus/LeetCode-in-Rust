// Tests for Problem 2409: Count Days Spent Together
// Java reference: src/test/java/g2401_2500/s2409_count_days_spent_together/SolutionTest.java

use leetcode_in_rust::s2409::count_days_spent_together::Solution;

#[test]
fn test_count_days_together() {
    assert_eq!(
        Solution::count_days_together(
            "08-15".to_string(),
            "08-18".to_string(),
            "08-16".to_string(),
            "08-19".to_string()
        ),
        3
    );
}

#[test]
fn test_count_days_together2() {
    assert_eq!(
        Solution::count_days_together(
            "10-01".to_string(),
            "10-31".to_string(),
            "11-01".to_string(),
            "12-31".to_string()
        ),
        0
    );
}

#[test]
fn test_count_days_together3() {
    assert_eq!(
        Solution::count_days_together(
            "09-01".to_string(),
            "10-19".to_string(),
            "06-19".to_string(),
            "10-20".to_string()
        ),
        49
    );
}

#[test]
fn test_count_days_together4() {
    assert_eq!(
        Solution::count_days_together(
            "08-06".to_string(),
            "12-08".to_string(),
            "02-04".to_string(),
            "09-01".to_string()
        ),
        27
    );
}

#[test]
fn test_count_days_together5() {
    assert_eq!(
        Solution::count_days_together(
            "03-05".to_string(),
            "07-14".to_string(),
            "04-14".to_string(),
            "09-21".to_string()
        ),
        92
    );
}
