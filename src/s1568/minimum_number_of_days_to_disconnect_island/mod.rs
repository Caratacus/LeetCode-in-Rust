// Problem 1568: Minimum Number of Days to Disconnect Island
// #Hard #Array #Depth_First_Search #Breadth_First_Search #Matrix
// #Big_O_Time_O(m*n)_Space_O(m*n)

pub struct Solution;

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut num_of_islands = 0;
        let mut has_articulation_point = false;
        let mut color = 1;
        let mut min_island_size = m * n;
        let mut time_grid = vec![vec![0i32; n]; m];
        let mut low = vec![vec![0i32; n]; m];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    num_of_islands += 1;
                    color += 1;
                    let mut articulation_points = Vec::new();
                    let mut island_size = 0;
                    Self::tarjan(
                        i as i32,
                        j as i32,
                        -1,
                        -1,
                        0,
                        &mut time_grid,
                        &mut low,
                        &mut grid,
                        &mut articulation_points,
                        color,
                        &mut island_size,
                    );
                    min_island_size = min_island_size.min(island_size);
                    if !articulation_points.is_empty() {
                        has_articulation_point = true;
                    }
                }
            }
        }

        if num_of_islands >= 2 {
            return 0;
        }
        if num_of_islands == 0 {
            return 0;
        }
        if num_of_islands == 1 && min_island_size == 1 {
            return 1;
        }
        if has_articulation_point {
            1
        } else {
            2
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn tarjan(
        x: i32,
        y: i32,
        prex: i32,
        prey: i32,
        time: i32,
        times: &mut Vec<Vec<i32>>,
        lows: &mut Vec<Vec<i32>>,
        grid: &mut Vec<Vec<i32>>,
        articulation_points: &mut Vec<i32>,
        color: i32,
        island_size: &mut usize,
    ) {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let x_idx = x as usize;
        let y_idx = y as usize;

        times[x_idx][y_idx] = time;
        lows[x_idx][y_idx] = time;
        grid[x_idx][y_idx] = color;
        *island_size += 1;
        let mut children = 0;

        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= m || ny >= n {
                continue;
            }
            let nx_idx = nx as usize;
            let ny_idx = ny as usize;

            if grid[nx_idx][ny_idx] == 1 {
                children += 1;
                Self::tarjan(
                    nx,
                    ny,
                    x,
                    y,
                    time + 1,
                    times,
                    lows,
                    grid,
                    articulation_points,
                    color,
                    island_size,
                );
                lows[x_idx][y_idx] = lows[x_idx][y_idx].min(lows[nx_idx][ny_idx]);
                if prex != -1 && lows[nx_idx][ny_idx] >= time {
                    articulation_points.push(x * m + y);
                }
            } else if (nx != prex || ny != prey) && grid[nx_idx][ny_idx] != 0 {
                lows[x_idx][y_idx] = lows[x_idx][y_idx].min(times[nx_idx][ny_idx]);
            }
        }

        if prex == -1 && children > 1 {
            articulation_points.push(x * m + y);
        }
    }
}
