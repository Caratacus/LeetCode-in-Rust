// Tests for Problem 0981: Time Based Key-Value Store
// Java reference: src/test/java/g0901_1000/s0981_time_based_key_value_store/SolutionTest.java

use leetcode_in_rust::s0981::time_based_key_value_store::TimeMap;

#[test]
fn test_time_map() {
    let mut time_map = TimeMap::new();
    time_map.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(time_map.get("foo".to_string(), 1), "bar");
    assert_eq!(time_map.get("foo".to_string(), 3), "bar");
    time_map.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(time_map.get("foo".to_string(), 4), "bar2");
    assert_eq!(time_map.get("foo".to_string(), 5), "bar2");
}
