// Problem 2285: maximum total importance of roads

pub struct Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut degree = vec![0i64; n];
        for road in &roads {
            degree[road[0] as usize] += 1;
            degree[road[1] as usize] += 1;
        }
        degree.sort();
        degree
            .iter()
            .enumerate()
            .map(|(i, &d)| d * ((i + 1) as i64))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumImportance()
    //   assertThat(
    //   new Solution()
    //   .maximumImportance(
    //   5, new int[][] {{0, 1}, {1, 2}, {2, 3}, {0, 2}, {1, 3}, {2, 4}}),
    //   equalTo(43L));
    #[test]
    fn test_maximum_importance() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maximumImportance2()
    //   assertThat(
    //   new Solution().maximumImportance(5, new int[][] {{0, 3}, {2, 4}, {1, 3}}),
    //   equalTo(20L));
    #[test]
    fn test_maximum_importance2() {
        // TODO: 翻译 Java 测试
    }
}
