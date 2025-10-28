package leetcode_grind;

public class Day1074 {
    // https://leetcode.com/problems/make-array-elements-equal-to-zero/description/?envType=daily-question&envId=2025-10-28
    static class Solution1 {
        public int countValidSelections(int[] nums) {
            int count = 0;
            int nonZeros = 0;
            int n = nums.length;
            for (int x : nums)
                if (x > 0)
                    nonZeros++;
            for (int i = 0; i < n; i++) {
                if (nums[i] == 0) {
                    if (isValid(nums, nonZeros, i, -1))
                        count++;
                    if (isValid(nums, nonZeros, i, 1))
                        count++;
                }
            }
            return count;
        }

        boolean isValid(int[] nums, int nonZeros, int start, int direction) {
            int n = nums.length;
            int curr = start;
            int[] temp = nums.clone();
            while (nonZeros > 0 && curr >= 0 && curr < n) {
                if (temp[curr] > 0) {
                    temp[curr]--;
                    direction *= -1;
                    if (temp[curr] == 0)
                        nonZeros--;
                }
                curr += direction;
            }
            return nonZeros == 0;
        }
    }

    static class Solution2 {
        public int countValidSelections(int[] nums) {
            int n = nums.length;
            int ans = 0;
            int sum = 0;
            for (int x : nums) {
                sum += x;
            }

            int leftSum = 0;
            int rightSum = sum;
            for (int i = 0; i < n; i++) {
                if (nums[i] == 0) {
                    if (leftSum - rightSum >= 0 && leftSum - rightSum <= 1) {
                        ans++;
                    }
                    if (rightSum - leftSum >= 0 && rightSum - leftSum <= 1) {
                        ans++;
                    }
                } else {
                    leftSum += nums[i];
                    rightSum -= nums[i];
                }
            }
            return ans;
        }
    }
}
