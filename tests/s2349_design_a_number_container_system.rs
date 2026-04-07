// Tests for Problem 2349: Design a Number Container System
// Java reference: src/test/java/g2301_2400/s2349_design_a_number_container_system/NumberContainersTest.java

use leetcode_in_rust::s2349::design_a_number_container_system::NumberContainers;

#[test]
fn test_number_containers() {
    let mut nc = NumberContainers::new();
    // There is no index that is filled with number 10. Therefore, we return -1.
    assert_eq!(nc.find(10), -1);
    // Your container at index 2 will be filled with number 10.
    nc.change(2, 10);
    // Your container at index 1 will be filled with number 10.
    nc.change(1, 10);
    // Your container at index 3 will be filled with number 10.
    nc.change(3, 10);
    // Your container at index 5 will be filled with number 10.
    nc.change(5, 10);
    // Number 10 is at the indices 1, 2, 3, and 5. Since the smallest index that is filled with 10 is 1, we return 1.
    assert_eq!(nc.find(10), 1);
    // Your container at index 1 will be filled with number 20. Note that index 1 was filled with 10 and then replaced with 20.
    nc.change(1, 20);
    // Number 10 is at the indices 2, 3, and 5. The smallest index that is filled with 10 is 2.
    assert_eq!(nc.find(10), 2);
}
