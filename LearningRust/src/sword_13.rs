use std::collections::HashSet;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        Solution::dfs(0, 0, &mut visited, 0, 0, k, m, n)
    }
    pub fn dfs(i: i32, j: i32, visited: &mut HashSet<(i32, i32)>, sum_i: i32, sum_j: i32, k: i32, m: i32, n: i32) -> i32 {
        if visited.contains(&(i, j)) || (sum_i + sum_j) > k || i >= m || j >= n {
            return 0;
        }
        let mut new_sum_i = 0;
        let mut new_sum_j = 0;
        if i % 10 == 9 {
            new_sum_i = sum_i - 8;
        } else {
            new_sum_i = sum_i + 1;
        }
        if j % 10 == 9 {
            new_sum_j = sum_j - 8;
        } else {
            new_sum_j = sum_j + 1;
        }
        visited.insert((i, j));
        let res = 1 + Solution::dfs(i + 1, j, visited, new_sum_i, sum_j, k, m, n) + Solution::dfs(i, j + 1, visited, sum_i, new_sum_j, k, m, n);
        res
    }
}
//python3
//class Solution:
//     def movingCount(self, m: int, n: int, k: int) -> int:
//         def dfs(visited, i, j, si, sj):
//             if (i, j) in visited or si + sj > k or i > m - 1 or j > n - 1:
//                 return 0
//             else:
//                 visited.add((i, j))
//                 return 1 + dfs(visited, i + 1, j, si + 1 if(i + 1) % 10 else si - 8, sj) + dfs(visited, i, j + 1, si, sj  + 1 if(j + 1) % 10 else sj - 8)
//
//         visited= set()
//         return dfs(visited, 0, 0, 0, 0)