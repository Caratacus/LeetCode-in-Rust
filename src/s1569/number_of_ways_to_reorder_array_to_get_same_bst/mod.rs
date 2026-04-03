// Problem 1569: Number of Ways to Reorder Array to Get Same BST
// #Hard #Array #Dynamic_Programming #Math #Tree #Binary_Tree
// #Big_O_Time_O(n^2)_Space_O(n^2)

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut fact = vec![1i64; 1001];
        for i in 1..=1000 {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        let mut root = Some(Box::new(TreeNode::new(nums[0])));
        for i in 1..n {
            Self::add_in_tree(nums[i], &mut root);
        }

        ((Self::calc_perms(&root, &fact).perm - 1 + MOD) % MOD) as i32
    }

    fn calc_perms(node: &Option<Box<TreeNode>>, fact: &[i64]) -> TreeInfo {
        if node.is_none() {
            return TreeInfo { num_of_nodes: 0, perm: 1 };
        }
        let n = node.as_ref().unwrap();

        let left = Self::calc_perms(&n.left, fact);
        let right = Self::calc_perms(&n.right, fact);

        let tot_nodes = left.num_of_nodes + right.num_of_nodes + 1;

        let mod_div = Self::get_mod_division(
            fact[(tot_nodes - 1) as usize],
            fact[left.num_of_nodes as usize],
            fact[right.num_of_nodes as usize],
        );

        let perms = if tot_nodes == 1 {
            1
        } else {
            (((left.perm * right.perm) % MOD) * mod_div) % MOD
        };

        TreeInfo {
            num_of_nodes: tot_nodes,
            perm: perms,
        }
    }

    fn get_mod_division(a: i64, b1: i64, b2: i64) -> i64 {
        let b = (b1 * b2) % MOD;
        let inv = Self::get_inverse(b, MOD);
        (inv * a) % MOD
    }

    fn get_inverse(b: i64, m: i64) -> i64 {
        let inv = Self::get_inverse_extended(b, m);
        ((inv.x % m) + m) % m
    }

    fn get_inverse_extended(a: i64, b: i64) -> Inverse {
        if a == 0 {
            return Inverse { x: 0, y: 1 };
        }
        let inv = Self::get_inverse_extended(b % a, a);
        let x1 = inv.y - ((b / a) * inv.x);
        let y1 = inv.x;
        Inverse { x: x1, y: y1 }
    }

    fn add_in_tree(x: i32, node: &mut Option<Box<TreeNode>>) {
        if let Some(n) = node {
            if x < n.val {
                if n.left.is_none() {
                    n.left = Some(Box::new(TreeNode::new(x)));
                } else {
                    Self::add_in_tree(x, &mut n.left);
                }
            } else if x > n.val {
                if n.right.is_none() {
                    n.right = Some(Box::new(TreeNode::new(x)));
                } else {
                    Self::add_in_tree(x, &mut n.right);
                }
            }
        }
    }
}

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct TreeInfo {
    num_of_nodes: i64,
    perm: i64,
}

struct Inverse {
    x: i64,
    y: i64,
}
