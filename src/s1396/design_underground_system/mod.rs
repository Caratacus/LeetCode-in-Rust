// Problem 1396: design underground system

pub struct UndergroundSystem {}

impl UndergroundSystem {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_station(&self) -> String {
        todo!()
    }

    pub fn get_time(&self) -> i32 {
        todo!()
    }

    pub fn check_in(&mut self, id: i32, station_name: String, t: i32) -> () {
        todo!()
    }

    pub fn check_out(&mut self, id: i32, station_name: String, t: i32) -> () {
        todo!()
    }

    pub fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void undergroundSystem()
    //   UndergroundSystem undergroundSystem = new UndergroundSystem();
    //   undergroundSystem.checkIn(45, "Leyton", 3);
    //   undergroundSystem.checkIn(32, "Paradise", 8);
    //   undergroundSystem.checkIn(27, "Leyton", 10);
    //   // Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
    //   ... (16 more lines)
    #[test]
    fn test_underground_system() {
        // TODO: 翻译 Java 测试
    }

    // Java: void undergroundSystem2()
    //   UndergroundSystem undergroundSystem = new UndergroundSystem();
    //   undergroundSystem.checkIn(10, "Leyton", 3);
    //   // Customer 10 "Leyton" -> "Paradise" in 8-3 = 5
    //   undergroundSystem.checkOut(10, "Paradise", 8);
    //   // return 5.00000, (5) / 1 = 5
    //   ... (12 more lines)
    #[test]
    fn test_underground_system2() {
        // TODO: 翻译 Java 测试
    }
}
