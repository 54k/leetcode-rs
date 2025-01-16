package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day789 {
    // https://leetcode.com/problems/bitwise-xor-of-all-pairings/description/?envType=daily-question&envId=2025-01-16
    static class Solution1 {
        public int xorAllNums(int[] nums1, int[] nums2) {
            int len1 = nums1.length;
            int len2 = nums2.length;
            Map<Integer, Integer> freq = new HashMap<>();
            for (int num : nums1) {
                freq.put(num, freq.getOrDefault(num, 0) + len2);
            }
            for (int num : nums2) {
                freq.put(num, freq.getOrDefault(num, 0) + len1);
            }
            int ans = 0;
            for (int num : freq.keySet()) {
                if (freq.get(num) % 2 == 1) {
                    ans ^= num;
                }
            }
            return ans;
        }
    }

}
