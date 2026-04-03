// Tests for Problem 1603: Design Parking System
// Java reference: src/test/java/g1601_1700/s1603_design_parking_system/ParkingSystemTest.java

use leetcode_in_rust::s1603::design_parking_system::ParkingSystem;

#[test]
fn test_parking_system_test() {
    let mut parking_system = ParkingSystem::new(1, 1, 0);
    assert_eq!(parking_system.add_car(1), true);
    assert_eq!(parking_system.add_car(2), true);
    assert_eq!(parking_system.add_car(3), false);
    assert_eq!(parking_system.add_car(1), false);
}
