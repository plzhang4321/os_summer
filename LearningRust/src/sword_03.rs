impl Solution {
    pub fn find_repeat_number(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let j = nums[i] as usize;
            if i != j && nums[i] == nums[j] {
                return nums[i];
            }
            nums.swap(i, j);
        }
        -1
    }
}
//python3
//class Solution:
//     def findRepeatNumber(self, nums: [int]) -> int:
//         i = 0
//         while i < len(nums):
//             if nums[i] == i:
//                 i += 1
//                 continue
//             if nums[nums[i]] == nums[i]: return nums[i]
//             nums[nums[i]], nums[i] = nums[i], nums[nums[i]]
//         return -1