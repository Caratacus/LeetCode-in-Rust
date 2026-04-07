// Tests for Problem 2418: Sort the People
// Java reference: src/test/java/g2401_2500/s2418_sort_the_people/SolutionTest.java

use leetcode_in_rust::s2418::sort_the_people::Solution;

#[test]
fn test_sort_people() {
    assert_eq!(
        Solution::sort_people(
            vec![String::from("Mary"), String::from("John"), String::from("Emma")],
            vec![180, 165, 170]
        ),
        vec![String::from("Mary"), String::from("Emma"), String::from("John")]
    );
}

#[test]
fn test_sort_people2() {
    assert_eq!(
        Solution::sort_people(
            vec![String::from("Alice"), String::from("Bob"), String::from("Bob")],
            vec![155, 185, 150]
        ),
        vec![String::from("Bob"), String::from("Alice"), String::from("Bob")]
    );
}
