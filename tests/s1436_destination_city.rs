// Tests for Problem 1436: Destination City
// Java reference: src/test/java/g1401_1500/s1436_destination_city/SolutionTest.java

use leetcode_in_rust::s1436::destination_city::Solution;

#[test]
fn test_dest_city() {
    assert_eq!(
        Solution::dest_city(vec![
            vec!["London".to_string(), "New York".to_string()],
            vec!["New York".to_string(), "Lima".to_string()],
            vec!["Lima".to_string(), "Sao Paulo".to_string()],
        ]),
        "Sao Paulo"
    );
}

#[test]
fn test_dest_city2() {
    assert_eq!(
        Solution::dest_city(vec![
            vec!["B".to_string(), "C".to_string()],
            vec!["D".to_string(), "B".to_string()],
            vec!["C".to_string(), "A".to_string()],
        ]),
        "A"
    );
}

#[test]
fn test_dest_city3() {
    assert_eq!(
        Solution::dest_city(vec![vec!["A".to_string(), "Z".to_string()]]),
        "Z"
    );
}
