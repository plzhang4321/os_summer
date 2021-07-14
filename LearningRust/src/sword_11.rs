impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, numbers.len() - 1);
        while l < r {
            let mut m = ((r + l) / 2) as usize;
            if numbers[m] > numbers[r] {
                l = m + 1;
            } else {
                if numbers[m] < numbers[r] {
                    r = m;
                } else {
                    r -= 1;
                }
            }
        }
        numbers[l]
    }
}
//python3
//class Solution:
//     def minArray(self, numbers: [int]) -> int:
//         i, j = 0, len(numbers) - 1
//         while i < j:
//             m = (i + j) // 2
//             if numbers[m] > numbers[j]: i = m + 1
//             elif numbers[m] < numbers[j]: j = m
//             else: j -= 1
//         return numbers[i]