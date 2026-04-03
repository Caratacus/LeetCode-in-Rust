// Tests for Problem 1604: Alert Using Same Key-Card Three or More Times in a One Hour Period
// Java reference: src/test/java/g1601_1700/s1604_alert_using_same_key_card_three_or_more_times_in_a_one_hour_period/SolutionTest.java

use leetcode_in_rust::s1604::alert_using_same_key_card_three_or_more_times_in_a_one_hour_period::Solution;

#[test]
fn test_alert_names() {
    assert_eq!(
        Solution::alert_names(
            vec![
                "daniel".to_string(),
                "daniel".to_string(),
                "daniel".to_string(),
                "luis".to_string(),
                "luis".to_string(),
                "luis".to_string(),
                "luis".to_string()
            ],
            vec![
                "10:00".to_string(),
                "10:20".to_string(),
                "10:40".to_string(),
                "11:00".to_string(),
                "11:20".to_string(),
                "11:40".to_string(),
                "12:00".to_string()
            ]
        ),
        vec!["daniel".to_string()]
    );
}

#[test]
fn test_alert_names2() {
    assert_eq!(
        Solution::alert_names(
            vec![
                "alice".to_string(),
                "alice".to_string(),
                "alice".to_string(),
                "bob".to_string(),
                "bob".to_string(),
                "bob".to_string(),
                "bob".to_string()
            ],
            vec![
                "12:01".to_string(),
                "12:00".to_string(),
                "12:02".to_string(),
                "12:02".to_string(),
                "12:30".to_string(),
                "12:32".to_string(),
                "13:31".to_string()
            ]
        ),
        vec!["bob".to_string()]
    );
}
