// Tests for Problem 2306: Naming a Company
// Java reference: src/test/java/g2301_2400/s2306_naming_a_company/SolutionTest.java

use leetcode_in_rust::s2306::naming_a_company::Solution;

#[test]
fn test_distinct_names() {
    assert_eq!(
        Solution::distinct_names(vec![
            String::from("coffee"),
            String::from("donuts"),
            String::from("time"),
            String::from("toffee")
        ]),
        6
    );
}

#[test]
fn test_distinct_names2() {
    assert_eq!(
        Solution::distinct_names(vec![String::from("lack"), String::from("back")]),
        0
    );
}
