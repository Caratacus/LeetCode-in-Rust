// Problem 3108: Minimum Cost Walk in Weighted Graph
// #Hard #Array #Bit_Manipulation #Graph #Union_Find
// #2024_04_11_Time_3_ms_(100.00%)_Space_118.6_MB_(21.36%)

pub struct Solution;

impl Solution {
    fn find_parent(node: usize, parent: &[i32]) -> usize {
        let mut node = node;
        while parent[node] != node as i32 {
            node = parent[node] as usize;
        }
        node
    }

    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut parent: Vec<i32> = (0..n as i32).collect();
        let mut bitwise: Vec<i32> = vec![-1; n];
        let mut size: Vec<i32> = vec![1; n];

        for edge in &edges {
            let node1 = edge[0] as usize;
            let node2 = edge[1] as usize;
            let weight = edge[2];
            let parent1 = Self::find_parent(node1, &parent);
            let parent2 = Self::find_parent(node2, &parent);

            if parent1 == parent2 {
                bitwise[parent1] &= weight;
            } else {
                let bitwise_val;
                let check1 = bitwise[parent1] == -1;
                let check2 = bitwise[parent2] == -1;
                if check1 && check2 {
                    bitwise_val = weight;
                } else if check1 {
                    bitwise_val = weight & bitwise[parent2];
                } else if check2 {
                    bitwise_val = weight & bitwise[parent1];
                } else {
                    bitwise_val = weight & bitwise[parent1] & bitwise[parent2];
                }

                if size[parent1] >= size[parent2] {
                    parent[parent2] = parent1 as i32;
                    size[parent1] += size[parent2];
                    bitwise[parent1] = bitwise_val;
                } else {
                    parent[parent1] = parent2 as i32;
                    size[parent2] += size[parent1];
                    bitwise[parent2] = bitwise_val;
                }
            }
        }

        let query_len = query.len();
        let mut result = vec![0; query_len];
        for i in 0..query_len {
            let start = query[i][0] as usize;
            let end = query[i][1] as usize;
            let parent_start = Self::find_parent(start, &parent);
            let parent_end = Self::find_parent(end, &parent);

            if start == end {
                result[i] = 0;
            } else if parent_start == parent_end {
                result[i] = bitwise[parent_start];
            } else {
                result[i] = -1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumCost()
    #[test]
    fn test_minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(
                5,
                vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
                vec![vec![0, 3], vec![3, 4]]
            ),
            vec![1, -1]
        );
    }

    // Java: void minimumCost2()
    #[test]
    fn test_minimum_cost2() {
        assert_eq!(
            Solution::minimum_cost(
                3,
                vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
                vec![vec![1, 2]]
            ),
            vec![0]
        );
    }
}
