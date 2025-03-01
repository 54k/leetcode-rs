package leetcode_grind;

public class Day833 {
    // https://leetcode.com/problems/apply-operations-to-an-array/description/?envType=daily-question&envId=2025-03-01
    static class Solution1 {
        public int[] applyOperations(int[] nums) {
            int n = nums.length;
            for (int i = 0; i < n - 1; i++) {
                if (nums[i] == nums[i + 1]) {
                    nums[i] *= 2;
                    nums[i + 1] = 0;
                }
            }

            for (int i = 0, j = 0; i < n; i++) {
                if (nums[i] != 0) {
                    int t = nums[i];
                    nums[i] = nums[j];
                    nums[j++] = t;
                }
            }

            return nums;
        }
    }

    static class Solution2 {
        public int[] applyOperations(int[] nums) {
            int n = nums.length;
            int writeIndex = 0;

            for (int index = 0; index < n; index++) {
                if (index < n - 1 && nums[index] == nums[index + 1] && nums[index] != 0) {
                    nums[index] *= 2;
                    nums[index + 1] = 0;
                }

                if (nums[index] != 0) {
                    if (index != writeIndex) {
                        int temp = nums[index];
                        nums[index] = nums[writeIndex];
                        nums[writeIndex] = temp;
                    }
                    writeIndex++;
                }
            }

            return nums;
        }
    }
}
