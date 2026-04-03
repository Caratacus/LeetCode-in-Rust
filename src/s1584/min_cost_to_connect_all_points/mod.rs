// Problem 1584: Min Cost to Connect All Points
// #Medium #Array #Union_Find #Minimum_Spanning_Tree
// #Big_O_Time_O(n^2_log_n)_Space_O(n)

use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let v = points.len();
        if v == 2 {
            return Self::get_distance(&points[0], &points[1]);
        }

        let mut mst = vec![false; v];
        let mut dist = vec![i32::MAX / 2; v];
        let mut parent = vec![-1i32; v];
        dist[0] = 0;
        parent[0] = 0;

        // Use a simple approach - Prim's algorithm
        for _ in 0..v {
            // Find the minimum distance vertex not in MST
            let mut min_dist = i32::MAX;
            let mut u = 0;
            for j in 0..v {
                if !mst[j] && dist[j] < min_dist {
                    min_dist = dist[j];
                    u = j;
                }
            }

            mst[u] = true;

            // Update distances for adjacent vertices
            for i in 0..v {
                if !mst[i] {
                    let d = Self::get_distance(&points[u], &points[i]);
                    if d < dist[i] {
                        dist[i] = d;
                        parent[i] = u as i32;
                    }
                }
            }
        }

        let mut cost = 0;
        for i in 1..parent.len() {
            cost += Self::get_distance(&points[parent[i] as usize], &points[i]);
        }
        cost
    }

    fn get_distance(p1: &[i32], p2: &[i32]) -> i32 {
        (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
    }
}
