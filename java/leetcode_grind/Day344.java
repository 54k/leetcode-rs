package leetcode_grind;

import java.util.HashMap;

public class Day344 {
    // https://leetcode.com/problems/next-greater-element-i/description/
    static class Solution1 {
        public int[] nextGreaterElementBruteForce(int[] nums1, int[] nums2) {
            var res = new int[nums1.length];
            int j;

            for (var i = 0; i < nums1.length; i++) {
                var found = false;
                for (j = 0; j < nums2.length; j++) {
                    if (found && nums2[j] > nums1[i]) {
                        res[i] = nums2[j];
                        break;
                    }

                    if (nums2[j] == nums1[i]) {
                        found = true;
                    }
                }
                if (j == nums2.length) {
                    res[i] = -1;
                }
            }

            return res;
        }

        public int[] nextGreaterElement(int[] nums1, int[] nums2) {
            var hash = new HashMap<Integer, Integer>();
            for (var i = 0; i < nums2.length; i++) {
                hash.put(nums2[i], i);
            }

            var ans = new int[nums1.length];
            int j;

            for (int i = 0; i < nums1.length; i++) {
                for (j = hash.get(nums1[i]) + 1; j < nums2.length; j++) {
                    if (nums1[i] < nums2[j]) {
                        ans[i] = nums2[j];
                        break;
                    }
                }
                if (j == nums2.length) {
                    ans[i] = -1;
                }
            }

            return ans;
        }
    }
}
