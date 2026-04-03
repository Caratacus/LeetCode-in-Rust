// Problem 1591: Strange Printer II
// #Hard #Array #Matrix #Graph #Topological_Sort
// #Big_O_Time_O(m*n)_Space_O(m*n)

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let m = target_grid.len();
        let n = target_grid[0].len();

        // color_bound[color] = [min_i, min_j, max_i, max_j]
        let mut color_bound: Vec<[i32; 4]> = vec![[i32::MAX, i32::MAX, i32::MIN, i32::MIN]; 61];
        let mut colors: HashSet<i32> = HashSet::new();

        // Find the color range for each color
        for i in 0..m {
            for j in 0..n {
                let color = target_grid[i][j] as usize;
                color_bound[color][0] = color_bound[color][0].min(i as i32);
                color_bound[color][1] = color_bound[color][1].min(j as i32);
                color_bound[color][2] = color_bound[color][2].max(i as i32);
                color_bound[color][3] = color_bound[color][3].max(j as i32);
                colors.insert(target_grid[i][j]);
            }
        }

        let mut printed = vec![false; 61];
        let mut visited = vec![vec![false; n]; m];

        // DFS all the colors, skip the color already be printed
        for &color in &colors {
            if printed[color as usize] {
                continue;
            }
            if !Self::dfs(
                &target_grid,
                &mut printed,
                &color_bound,
                &mut visited,
                color as usize,
            ) {
                return false;
            }
        }

        true
    }

    fn dfs(
        target_grid: &[Vec<i32>],
        printed: &mut [bool],
        color_bound: &[[i32; 4]],
        visited: &mut [Vec<bool>],
        color: usize,
    ) -> bool {
        printed[color] = true;

        for i in color_bound[color][0]..=color_bound[color][2] {
            for j in color_bound[color][1]..=color_bound[color][3] {
                if visited[i as usize][j as usize] {
                    continue;
                }

                let current_color = target_grid[i as usize][j as usize] as usize;
                if current_color != color {
                    if printed[current_color] {
                        return false;
                    }
                    if !Self::dfs(target_grid, printed, color_bound, visited, current_color) {
                        return false;
                    }
                }
                visited[i as usize][j as usize] = true;
            }
        }
        true
    }
}
