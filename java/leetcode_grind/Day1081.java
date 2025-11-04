package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day1081 {
    // https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/description/?envType=daily-question&envId=2025-11-04
    static class Solution1 {
        public int[] findXSum(int[] nums, int k, int x) {
            int n = nums.length;
            int[] ans = new int[n - k + 1];
            for (int i = 0; i <= n - k; i++) {
                Map<Integer, Integer> cnt = new HashMap<>();
                for (int j = i; j < i + k; j++) {
                    cnt.put(nums[j], cnt.getOrDefault(nums[j], 0) + 1);
                }

                List<int[]> freq = new ArrayList<>();
                for (Map.Entry<Integer, Integer> entry : cnt.entrySet()) {
                    freq.add(new int[] { entry.getValue(), entry.getKey() });
                }

                freq.sort((a, b) -> b[0] != a[0] ? b[0] - a[0] : b[1] - a[1]);
                int xsum = 0;
                for (int j = 0; j < x && j < freq.size(); ++j) {
                    xsum += freq.get(j)[0] * freq.get(j)[1];
                }
                ans[i] = xsum;
            }
            return ans;
        }
    }
}