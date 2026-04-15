// Problem 3386: button with longest push time

pub struct Solution;

impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut max_time = 0;
        let mut last = 0;
        for event in &events {
            let diff = event[1] - last;
            if diff > max_time {
                max_time = diff;
                ans = event[0];
            } else if diff == max_time {
                ans = ans.min(event[0]);
            }
            last = event[1];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void buttonWithLongestTime()
    //   assertThat(
    //   new Solution().buttonWithLongestTime(new int[][] {{1, 2}, {2, 5}, {3, 9}, {1, 15}}),
    //   equalTo(1));
    #[test]
    fn test_button_with_longest_time() {
        // TODO: 翻译 Java 测试
    }

    // Java: void buttonWithLongestTime2()
    //   assertThat(
    //   new Solution().buttonWithLongestTime(new int[][] {{10, 5}, {1, 7}}), equalTo(10));
    #[test]
    fn test_button_with_longest_time2() {
        // TODO: 翻译 Java 测试
    }
}
