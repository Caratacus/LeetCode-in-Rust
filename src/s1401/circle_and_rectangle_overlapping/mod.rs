// Problem 1401: circle and rectangle overlapping

pub struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void checkOverlap()
    //   assertThat(new Solution().checkOverlap(1, 0, 0, 1, -1, 3, 1), equalTo(true));
    #[test]
    fn test_check_overlap() {
        // TODO: 翻译 Java 测试
    }

    // Java: void checkOverlap2()
    //   assertThat(new Solution().checkOverlap(1, 1, 1, 1, -3, 2, -1), equalTo(false));
    #[test]
    fn test_check_overlap2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void checkOverlap3()
    //   assertThat(new Solution().checkOverlap(1, 0, 0, -1, 0, 0, 1), equalTo(true));
    #[test]
    fn test_check_overlap3() {
        // TODO: 翻译 Java 测试
    }
}
