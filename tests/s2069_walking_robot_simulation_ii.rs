// Tests for Problem 2069: Walking Robot Simulation II
// Java reference: src/test/java/g2001_2100/s2069_walking_robot_simulation_ii/SolutionTest.java

use leetcode_in_rust::s2069::walking_robot_simulation_ii::Robot;

#[test]
fn test_robot() {
    let mut robot = Robot::new(6, 3);
    robot.step(2);
    assert_eq!(robot.get_pos(), vec![2, 0]);
    assert_eq!(robot.get_dir(), "East");
    robot.step(2);
    assert_eq!(robot.get_pos(), vec![4, 0]);
    assert_eq!(robot.get_dir(), "East");
    robot.step(200000);
    assert_eq!(robot.get_pos(), vec![4, 0]);
    assert_eq!(robot.get_dir(), "South");
}

#[test]
fn test_robot2() {
    let mut robot = Robot::new(20, 17);
    robot.step(200000);
    assert_eq!(robot.get_pos(), vec![19, 16]);
    assert_eq!(robot.get_dir(), "South");
}

#[test]
fn test_robot3() {
    let mut robot = Robot::new(2, 2);
    robot.step(2);
    assert_eq!(robot.get_pos(), vec![2, 0]);
    assert_eq!(robot.get_dir(), "East");
}
