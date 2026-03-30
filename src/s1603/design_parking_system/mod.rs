// Problem 1603: design parking system

pub struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        todo!()
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void parkingSystemTest()
    //   ParkingSystem parkingSystem = new ParkingSystem(1, 1, 0);
    //   assertThat(parkingSystem.addCar(1), equalTo(true));
    //   assertThat(parkingSystem.addCar(2), equalTo(true));
    //   assertThat(parkingSystem.addCar(3), equalTo(false));
    //   assertThat(parkingSystem.addCar(1), equalTo(false));
    #[test]
    fn test_parking_system_test() {
        // TODO: 翻译 Java 测试
    }
}
