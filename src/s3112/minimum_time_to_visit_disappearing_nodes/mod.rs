// Problem 3112: minimum time to visit disappearing nodes
// #Medium #Array #Heap_Priority_Queue #Graph #Shortest_Path
// #2024_04_27_Time_10_ms_(100.00%)_Space_85.4_MB_(99.80%)

pub struct Solution;

impl Solution {
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut dist: Vec<i32> = vec![i32::MAX; n];
        dist[0] = 0;
        let mut exit = false;
        for _ in 0..n {
            if exit {
                break;
            }
            exit = true;
            for edge in &edges {
                let src = edge[0] as usize;
                let dest = edge[1] as usize;
                let cost = edge[2];
                if dist[src] != i32::MAX && dist[src] < disappear[src] && dist[src] + cost < dist[dest] {
                    exit = false;
                    dist[dest] = dist[src] + cost;
                }
                if dist[dest] != i32::MAX && dist[dest] < disappear[dest] && dist[dest] + cost < dist[src] {
                    exit = false;
                    dist[src] = dist[dest] + cost;
                }
            }
        }
        for i in 0..dist.len() {
            if dist[i] == i32::MAX || dist[i] >= disappear[i] {
                dist[i] = -1;
            }
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_time() {
        assert_eq!(
            Solution::minimum_time(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
                vec![1, 1, 5]
            ),
            vec![0, -1, 4]
        );
    }

    #[test]
    fn test_minimum_time2() {
        assert_eq!(
            Solution::minimum_time(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
                vec![1, 3, 5]
            ),
            vec![0, 2, 3]
        );
    }

    #[test]
    fn test_minimum_time3() {
        assert_eq!(
            Solution::minimum_time(2, vec![vec![0, 1, 1]], vec![1, 1]),
            vec![0, -1]
        );
    }
}
