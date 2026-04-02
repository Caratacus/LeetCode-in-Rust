// Tests for Problem 1268: Search Suggestions System
// Java reference: src/test/java/g1201_1300/s1268_search_suggestions_system/SolutionTest.java

use leetcode_in_rust::s1268::search_suggestions_system::Solution;

#[test]
fn test_suggested_products() {
    assert_eq!(
        Solution::suggested_products(
            vec![
                "mobile".to_string(),
                "mouse".to_string(),
                "moneypot".to_string(),
                "monitor".to_string(),
                "mousepad".to_string()
            ],
            "mouse".to_string()
        ),
        vec![
            vec!["mobile".to_string(), "moneypot".to_string(), "monitor".to_string()],
            vec!["mobile".to_string(), "moneypot".to_string(), "monitor".to_string()],
            vec!["mouse".to_string(), "mousepad".to_string()],
            vec!["mouse".to_string(), "mousepad".to_string()],
            vec!["mouse".to_string(), "mousepad".to_string()]
        ]
    );
}

#[test]
fn test_suggested_products2() {
    assert_eq!(
        Solution::suggested_products(vec!["havana".to_string()], "havana".to_string()),
        vec![
            vec!["havana".to_string()],
            vec!["havana".to_string()],
            vec!["havana".to_string()],
            vec!["havana".to_string()],
            vec!["havana".to_string()],
            vec!["havana".to_string()]
        ]
    );
}

#[test]
fn test_suggested_products3() {
    assert_eq!(
        Solution::suggested_products(
            vec![
                "bags".to_string(),
                "baggage".to_string(),
                "banner".to_string(),
                "box".to_string(),
                "cloths".to_string()
            ],
            "bags".to_string()
        ),
        vec![
            vec!["baggage".to_string(), "bags".to_string(), "banner".to_string()],
            vec!["baggage".to_string(), "bags".to_string(), "banner".to_string()],
            vec!["baggage".to_string(), "bags".to_string()],
            vec!["bags".to_string()]
        ]
    );
}
