fn main() {
    let i = 3 - 2;
}
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
    pub fn dfs(&mut board: Vec<Vec<char>>, chars: &Vec<char>, i: i32, j: i32, k: usize) -> bool {
        if k == chars.len() {
            return true;
        }
        if i < 0 || j < 0 || i == board.len() as i32 || j == board[0].len() as i32 || chars[k] != board[i as usize][j as usize] {
            return false;
        }
        board[i as usize][j as usize] = " ";
        let mut res = false;
        res = Solution::dfs(board, chars, i + 1, j, k + 1) || Solution::dfs(board, chars, i - 1, j, k + 1) || Solution::dfs(board, chars, i, j + 1, k + 1) || Solution::dfs(board, chars, i, j - 1, k + 1);
        board[i as usize][j as usize] = chars[k];
        return res;
    }
}