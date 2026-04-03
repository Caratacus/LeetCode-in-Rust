// Tests for Problem 1172: Dinner Plate Stacks
// Java reference: src/test/java/g1101_1200/s1172_dinner_plate_stacks/SolutionTest.java

use leetcode_in_rust::s1172::dinner_plate_stacks::DinnerPlates;

#[test]
fn test_dinner_plates_test() {
    let mut d = DinnerPlates::new(2);
    d.push(1);
    d.push(2);
    d.push(3);
    d.push(4);
    d.push(5);
    assert_eq!(d.pop_at_stack(0), 2);
    assert_eq!(d.pop_at_stack(0), 1);
    d.push(20);
    d.push(21);
    assert_eq!(d.pop_at_stack(0), 20);
    assert_eq!(d.pop_at_stack(2), 21);
    assert_eq!(d.pop(), 5);
    assert_eq!(d.pop(), 4);
    assert_eq!(d.pop(), 3);
    assert_eq!(d.pop(), 2);
    assert_eq!(d.pop(), 1);
}
