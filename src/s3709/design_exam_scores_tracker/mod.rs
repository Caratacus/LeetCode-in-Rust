// Problem 3709: design exam scores tracker

pub struct ExamTracker {}

impl ExamTracker {
    pub fn new() -> Self {
        todo!()
    }

    pub fn record(&mut self, time: i32, score: i32) -> () {
        todo!()
    }

    pub fn total_score(&mut self, start_time: i32, end_time: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void examTracker()
    //   ExamTracker examTracker = new ExamTracker();
    //   // Alice takes a new exam at time 1, scoring 98.
    //   examTracker.record(1, 98);
    //   // Between time 1 and time 1, Alice took 1 exam at time 1, scoring 98. The total score is
    //   // 98.
    //   ... (14 more lines)
    #[test]
    fn test_exam_tracker() {
        // TODO: 翻译 Java 测试
    }
}
