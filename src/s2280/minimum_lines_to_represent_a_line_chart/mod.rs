// Problem 2280: minimum lines to represent a line chart

pub struct Solution;

impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        let n = stock_prices.len();
        if n <= 1 {
            return 0;
        }
        if n == 2 {
            return 1;
        }
        let mut points: Vec<(i64, i64)> = stock_prices
            .iter()
            .map(|p| (p[0] as i64, p[1] as i64))
            .collect();
        points.sort_by_key(|p| p.0);
        let mut lines = 1;
        for i in 2..n {
            let (x1, y1) = points[i - 2];
            let (x2, y2) = points[i - 1];
            let (x3, y3) = points[i];
            // Cross product to check collinearity
            let cross = (x2 - x1) * (y3 - y2) - (y2 - y1) * (x3 - x2);
            if cross != 0 {
                lines += 1;
            }
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumLines()
    //   assertThat(
    //   new Solution()
    //   .minimumLines(
    //   new int[][] {
    //   {1, 7}, {2, 6}, {3, 5}, {4, 4}, {5, 4}, {6, 3}, {7, 2}, {8, 1}
    //   ... (2 more lines)
    #[test]
    fn test_minimum_lines() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumLines2()
    //   assertThat(
    //   new Solution().minimumLines(new int[][] {{3, 4}, {1, 2}, {7, 8}, {2, 3}}),
    //   equalTo(1));
    #[test]
    fn test_minimum_lines2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumLines3()
    //   assertThat(new Solution().minimumLines(new int[][] {{3, 4}}), equalTo(0));
    #[test]
    fn test_minimum_lines3() {
        // TODO: 翻译 Java 测试
    }
}
