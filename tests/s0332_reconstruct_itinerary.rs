// Tests for Problem 0332: Reconstruct Itinerary
// Java reference: src/test/java/g0301_0400/s0332_reconstruct_itinerary/SolutionTest.java

use leetcode_in_rust::s0332::reconstruct_itinerary::Solution;

#[test]
fn test_find_itinerary() {
    let tickets = vec![
        vec!["MUC".to_string(), "LHR".to_string()],
        vec!["JFK".to_string(), "MUC".to_string()],
        vec!["SFO".to_string(), "SJC".to_string()],
        vec!["LHR".to_string(), "SFO".to_string()],
    ];
    let expected = vec![
        "JFK".to_string(),
        "MUC".to_string(),
        "LHR".to_string(),
        "SFO".to_string(),
        "SJC".to_string(),
    ];
    assert_eq!(Solution::find_itinerary(tickets), expected);
}

#[test]
fn test_find_itinerary2() {
    let tickets = vec![
        vec!["JFK".to_string(), "SFO".to_string()],
        vec!["JFK".to_string(), "ATL".to_string()],
        vec!["SFO".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "JFK".to_string()],
        vec!["ATL".to_string(), "SFO".to_string()],
    ];
    let expected = vec![
        "JFK".to_string(),
        "ATL".to_string(),
        "JFK".to_string(),
        "SFO".to_string(),
        "ATL".to_string(),
        "SFO".to_string(),
    ];
    assert_eq!(Solution::find_itinerary(tickets), expected);
}
