// Problem 0855: exam room

pub struct ExamRoom {
    n: i32,
}

impl ExamRoom {
    pub fn new(n: i32) -> Self {
        Self { n }
    }

    pub fn seat(&mut self) -> i32 {
        todo!()
    }

    pub fn leave(&mut self, p: i32) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void examRoomTest()
    //
    //   ExamRoom examRoom = new ExamRoom(10);
    //   assertThat(examRoom.seat(), equalTo(0));
    //   assertThat(examRoom.seat(), equalTo(9));
    //   assertThat(examRoom.seat(), equalTo(4));
    //   ... (3 more lines)
    #[test]
    fn test_exam_room_test() {
        // TODO: 翻译 Java 测试
    }
}
