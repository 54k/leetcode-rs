package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;

public class Day594 {
    // https://leetcode.com/problems/intersection-of-two-arrays-ii/description/?envType=daily-question&envId=2024-07-02
    static class Solution1 {
        public int[] intersect(int[] nums1, int[] nums2) {
            if (nums1.length > nums2.length) {
                return intersect(nums2, nums1);
            }
            HashMap<Integer, Integer> m = new HashMap<>();
            for (int n : nums1) {
                m.put(n, m.getOrDefault(n, 0) + 1);
            }
            int k = 0;
            for (int n : nums2) {
                int cnt = m.getOrDefault(n, 0);
                if (cnt > 0) {
                    nums1[k++] = n;
                    m.put(n, cnt - 1);
                }
            }
            return Arrays.copyOfRange(nums1, 0, k);
        }
    }
}
