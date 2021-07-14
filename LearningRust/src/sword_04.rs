impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false
        }
        let (mut i, mut j) = (matrix.len() - 1, 0);
        while j < matrix[0].len() {
            if matrix[i][j] == target {
                return true
            }
            if matrix[i][j] > target {
                if i == 0 {
                    break
                }
                i -= 1
            } else {
                j += 1
            }
        }
        false
    }
}
//python3
//class Solution:
//def findNumberIn2DArray(self, matrix: List[List[int]], target: int) -> bool:
//i, j = len(matrix) - 1, 0
//while i >= 0 and j < len(matrix[0]):
//if matrix[i][j] > target: i -= 1
//elif matrix[i][j] < target: j += 1
//else: return True
//return False
