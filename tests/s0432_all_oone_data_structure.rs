// Tests for Problem 0432: All Oone Data Structure
// Java reference: src/test/java/g0401_0500/s0432_all_oone_data_structure/SolutionTest.java

use leetcode_in_rust::s0432::all_oone_data_structure::AllOne;

#[test]
fn test_all_one() {
    let mut all_one = AllOne::new();
    all_one.inc("hello".to_string());
    all_one.inc("hello".to_string());
    assert_eq!(all_one.get_max_key(), "hello");
    assert_eq!(all_one.get_min_key(), "hello");
    all_one.inc("leet".to_string());
    assert_eq!(all_one.get_max_key(), "hello");
    assert_eq!(all_one.get_min_key(), "leet");
}
