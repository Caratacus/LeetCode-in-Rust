// Problem 2446: determine if two events have conflict

pub struct Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void haveConflict()
    //   assertThat(
    //   new Solution()
    //   .haveConflict(
    //   new String[] {"01:15", "02:00"}, new String[] {"02:00", "03:00"}),
    //   equalTo(true));
    #[test]
    fn test_have_conflict() {
        // TODO: 翻译 Java 测试
    }

    // Java: void haveConflict2()
    //   assertThat(
    //   new Solution()
    //   .haveConflict(
    //   new String[] {"01:00", "02:00"}, new String[] {"01:20", "03:00"}),
    //   equalTo(true));
    #[test]
    fn test_have_conflict2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void haveConflict3()
    //   assertThat(
    //   new Solution()
    //   .haveConflict(
    //   new String[] {"10:00", "11:00"}, new String[] {"14:00", "15:00"}),
    //   equalTo(false));
    #[test]
    fn test_have_conflict3() {
        // TODO: 翻译 Java 测试
    }
}
