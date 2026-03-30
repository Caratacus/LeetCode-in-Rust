// Problem 0729: my calendar i

pub struct MyCalendar {}

struct Meeting {/* TODO */}

impl MyCalendar {
    pub fn new() -> Self {
        todo!()
    }

    pub fn compare_to(&mut self, another_meeting: Meeting) -> i32 {
        todo!()
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void myCalendarTest()
    //   MyCalendar myCalendar = new MyCalendar();
    //   assertThat(myCalendar.book(10, 20), equalTo(true));
    //   assertThat(myCalendar.book(15, 25), equalTo(false));
    //   assertThat(myCalendar.book(20, 30), equalTo(true));
    #[test]
    fn test_my_calendar_test() {
        // TODO: 翻译 Java 测试
    }
}
