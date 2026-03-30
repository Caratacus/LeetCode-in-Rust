// Problem 1845: seat reservation manager

pub struct SeatManager {
    n: i32,
}

impl SeatManager {
    pub fn new(n: i32) -> Self {
        todo!()
    }

    pub fn reserve(&mut self) -> i32 {
        todo!()
    }

    pub fn unreserve(&mut self, seat_number: i32) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void seatManagerTest()
    //   SeatManager seatManager = new SeatManager(5);
    //   assertThat(seatManager.reserve(), equalTo(1));
    //   assertThat(seatManager.reserve(), equalTo(2));
    //   seatManager.unreserve(2);
    //   assertThat(seatManager.reserve(), equalTo(2));
    //   ... (4 more lines)
    #[test]
    fn test_seat_manager_test() {
        // TODO: 翻译 Java 测试
    }
}
