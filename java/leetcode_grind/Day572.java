package leetcode_grind;

public class Day572 {
    // https://leetcode.com/problems/subarray-sums-divisible-by-k/description
    static class Solution1 {
        public int subarraysDivByK(int[] nums, int k) {
            var modGroup = new int[k];
            modGroup[0] = 1;
            var ans = 0;
            var sum = 0;
            for (int i = 0; i < nums.length; i++) {
                sum += (nums[i] % k + k) % k;
                sum %= k;
                ans += modGroup[sum];
                modGroup[sum]++;

            }
            return ans;
        }
    }
}
