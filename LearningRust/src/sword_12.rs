//python3
//class Solution:
//     def exist(self, board: List[List[str]], word: str) -> bool:
//         def dfs(i, j, k):
//             if k == len(word):
//                 return True
//             if not 0 <= i < len(board) or not 0 <= j < len(board[0]) or board[i][j] != word[k]:
//                 return False
//             board[i][j] = ""
//             res = dfs(i + 1, j, k + 1) or dfs(i, j + 1, k + 1) or dfs(i - 1, j, k + 1) or dfs(i, j - 1, k + 1)
//             board[i][j] = word[k]
//             return res
//         for i in range(len(board)):
//             for j in range(len(board[0])):
//                 res = dfs(i, j, 0)
//                 if res:
//                     return res
//         return False
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let chars: Vec<char> = word.chars().collect();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Solution::dfs(&mut board, &chars, i as i32, j as i32, 0) {
                    return true;
                }
            }
        }
        false
    }
    pub fn dfs(board: &mut Vec<Vec<char>>, chars: &Vec<char>, i: i32, j: i32, k: usize) -> bool {
        if k == chars.len() {
            return true;
        }
        if i < 0 || j < 0 || i == board.len() as i32 || j == board[0].len() as i32 || chars[k] != board[i as usize][j as usize] {
            return false;
        }
        board[i as usize][j as usize] = ' ';
        let res = Solution::dfs(board, chars, i + 1, j, k + 1) || Solution::dfs(board, chars, i - 1, j, k + 1) || Solution::dfs(board, chars, i, j + 1, k + 1) || Solution::dfs(board, chars, i, j - 1, k + 1);
        board[i as usize][j as usize] = chars[k];
        return res;
    }
}