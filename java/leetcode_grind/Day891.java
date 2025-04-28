package leetcode_grind;

public class Day891 {
    // https://leetcode.com/problems/count-subarrays-with-score-less-than-k/description/?envType=daily-question&envId=2025-04-28
    static class Solution1 {
        public long countSubarrays(int[] nums, long k) {
            int n = nums.length;
            long res = 0, total = 0;
            for (int i = 0, j = 0; j < n; j++) {
                total += nums[j];
                while (i <= j && total * (j - i + 1) >= k) {
                    total -= nums[i];
                    i++;
                }
                res += j - i + 1;
            }
            return res;
        }
    }
}
