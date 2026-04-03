// Tests for Problem 1452: People Whose List of Favorite Companies Is Not a Subset of Another List
// Java reference: src/test/java/g1401_1500/s1452_people_whose_list_of_favorite_companies_is_not_a_subset_of_another_list/SolutionTest.java

use leetcode_in_rust::s1452::people_whose_list_of_favorite_companies_is_not_a_subset_of_another_list::Solution;

#[test]
fn test_people_indexes() {
    let input = vec![
        vec!["leetcode".to_string(), "google".to_string(), "facebook".to_string()],
        vec!["google".to_string(), "microsoft".to_string()],
        vec!["google".to_string(), "facebook".to_string()],
        vec!["google".to_string()],
        vec!["amazon".to_string()],
    ];
    assert_eq!(Solution::people_indexes(input), vec![0, 1, 4]);
}

#[test]
fn test_people_indexes2() {
    let input = vec![
        vec!["leetcode".to_string(), "google".to_string(), "facebook".to_string()],
        vec!["leetcode".to_string(), "amazon".to_string()],
        vec!["facebook".to_string(), "google".to_string()],
    ];
    assert_eq!(Solution::people_indexes(input), vec![0, 1]);
}

#[test]
fn test_people_indexes3() {
    let input = vec![
        vec!["leetcode".to_string()],
        vec!["google".to_string()],
        vec!["facebook".to_string()],
        vec!["amazon".to_string()],
    ];
    assert_eq!(Solution::people_indexes(input), vec![0, 1, 2, 3]);
}
