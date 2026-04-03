// Tests for Problem 1396: Design Underground System
// Java reference: src/test/java/g1301_1400/s1396_design_underground_system/SolutionTest.java

use leetcode_in_rust::s1396::design_underground_system::UndergroundSystem;

#[test]
fn test_underground_system() {
    let mut system = UndergroundSystem::new();

    // Customer 45 "Leyton" at time 3
    system.check_in(45, "Leyton".to_string(), 3);

    // Customer 32 "Paradise" at time 8
    system.check_in(32, "Paradise".to_string(), 8);

    // Customer 27 "Leyton" at time 10
    system.check_in(27, "Leyton".to_string(), 10);

    // Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
    system.check_out(45, "Waterloo".to_string(), 15);

    // Customer 27 "Leyton" -> "Waterloo" in 20-10 = 10
    system.check_out(27, "Waterloo".to_string(), 20);

    // Customer 32 "Paradise" -> "Cambridge" in 22-8 = 14
    system.check_out(32, "Cambridge".to_string(), 22);

    // Average time "Paradise" -> "Cambridge" = 14.00000
    let avg = system.get_average_time("Paradise".to_string(), "Cambridge".to_string());
    assert!((avg - 14.0).abs() < 0.00001);

    // Customer 26 "St Pancras" at time 16
    system.check_in(26, "St Pancras".to_string(), 16);

    // Customer 26 "St Pancras" -> "Kings Cross" in 24-16 = 8
    system.check_out(26, "Kings Cross".to_string(), 24);

    // Average time "St Pancras" -> "Kings Cross" = 8.00000
    let avg2 = system.get_average_time("St Pancras".to_string(), "Kings Cross".to_string());
    assert!((avg2 - 8.0).abs() < 0.00001);

    // Average time "Leyton" -> "Waterloo" = (12+10)/2 = 11.00000
    let avg3 = system.get_average_time("Leyton".to_string(), "Waterloo".to_string());
    assert!((avg3 - 11.0).abs() < 0.00001);
}

#[test]
fn test_underground_system2() {
    let mut system = UndergroundSystem::new();

    // Customer 10 "Leyton" at time 3
    system.check_in(10, "Leyton".to_string(), 3);

    // Customer 10 "Leyton" -> "Paradise" in 8-3 = 5
    system.check_out(10, "Paradise".to_string(), 8);

    // return 5.00000, (5) / 1 = 5
    let avg = system.get_average_time("Leyton".to_string(), "Paradise".to_string());
    assert!((avg - 5.0).abs() < 0.00001);

    // Customer 5 "Leyton" at time 10
    system.check_in(5, "Leyton".to_string(), 10);

    // Customer 5 "Leyton" -> "Paradise" in 16-10 = 6
    system.check_out(5, "Paradise".to_string(), 16);

    // return 5.50000, (5 + 6) / 2 = 5.5
    let avg2 = system.get_average_time("Leyton".to_string(), "Paradise".to_string());
    assert!((avg2 - 5.5).abs() < 0.00001);

    // Customer 2 "Leyton" at time 21
    system.check_in(2, "Leyton".to_string(), 21);

    // Customer 2 "Leyton" -> "Paradise" in 30-21 = 9
    system.check_out(2, "Paradise".to_string(), 30);

    // return 6.66667, (5 + 6 + 9) / 3 = 6.66667
    let avg3 = system.get_average_time("Leyton".to_string(), "Paradise".to_string());
    assert!((avg3 - 20.0/3.0).abs() < 0.00001);
}
