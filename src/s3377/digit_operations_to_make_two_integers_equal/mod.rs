// Problem 3377: Digit Operations to Make Two Integers Equal
// #Medium #Math #Heap_Priority_Queue #Graph #Shortest_Path #Number_Theory

use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn min_operations(n: i32, m: i32) -> i32 {
        let limit = 100000;
        let mut sieve = vec![true; limit + 1];
        let mut visited = vec![false; limit];
        sieve[0] = false;
        sieve[1] = false;
        for i in 2..=((limit as f64).sqrt() as usize) {
            if sieve[i] {
                for j in (i * i..=limit).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
        if sieve[n as usize] {
            return -1;
        }
        // Use min-heap via Reverse ordering (BinaryHeap is max-heap by default)
        let mut pq: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        visited[n as usize] = true;
        pq.push((Reverse(n), n));
        while let Some((Reverse(cost), num)) = pq.pop() {
            if num == m {
                return cost;
            }
            let temp = num.to_string().into_bytes();
            for j in 0..temp.len() {
                let old = temp[j];
                for i in -1..=1 {
                    let digit = (old - b'0') as i32;
                    if (digit == 9 && i == 1) || (digit == 0 && i == -1) {
                        continue;
                    }
                    let mut new_bytes = temp.clone();
                    new_bytes[j] = ((digit + i) as u8 + b'0') as u8;
                    let newnum = String::from_utf8_lossy(&new_bytes).parse::<i32>().unwrap();
                    if !sieve[newnum as usize] && !visited[newnum as usize] {
                        visited[newnum as usize] = true;
                        pq.push((Reverse(cost + newnum), newnum));
                    }
                }
            }
        }
        -1
    }
}
