// Problem 3394: check if grid can be cut into sections

pub struct Solution;

impl Solution {
    pub fn check_valid_cuts(m: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let n = rectangles.len();
        // Check vertical cuts (y-axis)
        let mut start: Vec<u64> = rectangles.iter().map(|r| ((r[1] as u64) << 32) | (r[3] as u64)).collect();
        start.sort();
        if Self::validate(&start) {
            return true;
        }
        // Check horizontal cuts (x-axis)
        start = rectangles.iter().map(|r| ((r[0] as u64) << 32) | (r[2] as u64)).collect();
        start.sort();
        Self::validate(&start)
    }

    fn validate(arr: &[u64]) -> bool {
        let mut cut = 0;
        let mut max_end = (arr[0] & 0xFFFFFFFF) as i32;
        for &l in arr {
            let start = (l >> 32) as i32;
            if start >= max_end {
                cut += 1;
                if cut == 2 {
                    return true;
                }
            }
            max_end = max_end.max((l & 0xFFFFFFFF) as i32);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void checkValidCuts()
    //   assertThat(
    //   new Solution()
    //   .checkValidCuts(
    //   5,
    //   new int[][] {
    //   ... (3 more lines)
    #[test]
    fn test_check_valid_cuts() {
        // TODO: 翻译 Java 测试
    }

    // Java: void checkValidCuts2()
    //   assertThat(
    //   new Solution()
    //   .checkValidCuts(
    //   4,
    //   new int[][] {
    //   ... (3 more lines)
    #[test]
    fn test_check_valid_cuts2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void checkValidCuts3()
    //   assertThat(
    //   new Solution()
    //   .checkValidCuts(
    //   4,
    //   new int[][] {
    //   ... (7 more lines)
    #[test]
    fn test_check_valid_cuts3() {
        // TODO: 翻译 Java 测试
    }
}
