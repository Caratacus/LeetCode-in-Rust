// Tests for Problem 1825: Finding MK Average
// Java reference: src/test/java/g1801_1900/s1825_finding_mk_average/SolutionTest.java

use leetcode_in_rust::s1825::finding_mk_average::MKAverage;

#[test]
fn test_m_k_average() {
    let mut obj = MKAverage::new(3, 1);
    obj.add_element(3);
    obj.add_element(1);
    assert_eq!(obj.calculate_mk_average(), -1);
    obj.add_element(10);
    assert_eq!(obj.calculate_mk_average(), 3);
    obj.add_element(5);
    obj.add_element(5);
    obj.add_element(5);
    assert_eq!(obj.calculate_mk_average(), 5);
}

#[test]
fn test_m_k_average2() {
    let mut obj = MKAverage::new(6, 1);
    obj.add_element(3);
    obj.add_element(1);
    assert_eq!(obj.calculate_mk_average(), -1);
    obj.add_element(12);
    obj.add_element(5);
    obj.add_element(3);
    obj.add_element(4);
    assert_eq!(obj.calculate_mk_average(), 3);
}
