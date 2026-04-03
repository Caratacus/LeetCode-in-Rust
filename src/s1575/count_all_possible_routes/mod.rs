// Problem 1575: Count All Possible Routes
// #Hard #Array #Dynamic_Programming #Memoization
// #Big_O_Time_O(n*fuel)_Space_O(n*fuel)

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();
        let start = start as usize;
        let finish = finish as usize;
        let fuel = fuel as usize;

        let mut cache = vec![vec![None::<i64>; fuel + 1]; n];
        Self::dfs(&locations, start, finish, fuel, &mut cache) as i32
    }

    fn dfs(
        locations: &[i32],
        current: usize,
        finish: usize,
        fuel: usize,
        cache: &mut Vec<Vec<Option<i64>>>,
    ) -> i64 {
        if let Some(cached) = cache[current][fuel] {
            return cached;
        }

        let mut count = 0i64;
        if current == finish {
            count = (count + 1) % MOD;
        }

        for i in 0..locations.len() {
            if i != current {
                let cost = (locations[current] - locations[i]).unsigned_abs() as usize;
                if cost <= fuel {
                    count = (count + Self::dfs(locations, i, finish, fuel - cost, cache)) % MOD;
                }
            }
        }

        cache[current][fuel] = Some(count);
        count
    }
}
