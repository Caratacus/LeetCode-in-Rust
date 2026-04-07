// Problem 2290: minimum obstacle removal to reach corner

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dist = vec![vec![i32::MAX; n]; m];
        dist[0][0] = 0;
        let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
        deque.push_front((0, 0));
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some((r, c)) = deque.pop_front() {
            if r == m - 1 && c == n - 1 {
                return dist[r][c];
            }
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    let new_dist = dist[r][c] + grid[nr][nc];
                    if new_dist < dist[nr][nc] {
                        dist[nr][nc] = new_dist;
                        if grid[nr][nc] == 0 {
                            deque.push_front((nr, nc));
                        } else {
                            deque.push_back((nr, nc));
                        }
                    }
                }
            }
        }
        dist[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumObstacles()
    //   assertThat(
    //   new Solution().minimumObstacles(new int[][] {{0, 1, 1}, {1, 1, 0}, {1, 1, 0}}),
    //   equalTo(2));
    #[test]
    fn test_minimum_obstacles() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumObstacles2()
    //   assertThat(
    //   new Solution()
    //   .minimumObstacles(
    //   new int[][] {{0, 1, 0, 0, 0}, {0, 1, 0, 1, 0}, {0, 0, 0, 1, 0}}),
    //   equalTo(0));
    #[test]
    fn test_minimum_obstacles2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumObstacles3()
    //   assertThat(new Solution().minimumObstacles(new int[][] {{1}}), equalTo(0));
    #[test]
    fn test_minimum_obstacles3() {
        // TODO: 翻译 Java 测试
    }
}
