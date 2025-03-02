package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day834 {
    // https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together/description/?envType=weekly-question&envId=2025-03-01
    static class Solution1 {
        public int minSwaps(int[] data) {
            var total = Arrays.stream(data).sum();
            var curOnes = 0;
            var maxOnes = 0;
            for (int i = 0; i < data.length; i++) {
                curOnes += data[i];
                if (i >= total) {
                    curOnes -= data[i - total];
                }
                maxOnes = Math.max(maxOnes, curOnes);
            }
            return total - maxOnes;
        }
    }

    static class Solution2 {
        public int[][] mergeArrays(int[][] nums1, int[][] nums2) {
            int N1 = nums1.length, N2 = nums2.length;
            int ptr1 = 0, ptr2 = 0;

            List<int[]> mergedArray = new ArrayList<>();
            while (ptr1 < N1 && ptr2 < N2) {
                if (nums1[ptr1][0] == nums2[ptr2][0]) {
                    mergedArray.add(new int[] { nums1[ptr1][0], nums1[ptr1][1] + nums2[ptr2][1] });
                    ptr1++;
                    ptr2++;
                } else if (nums1[ptr1][0] < nums2[ptr2][0]) {
                    mergedArray.add(nums1[ptr1]);
                    ptr1++;
                } else {
                    mergedArray.add(nums2[ptr2]);
                    ptr2++;
                }
            }

            while (ptr1 < N1) {
                mergedArray.add(nums1[ptr1]);
                ptr1++;
            }

            while (ptr2 < N2) {
                mergedArray.add(nums2[ptr2]);
                ptr2++;
            }

            int[][] result = new int[mergedArray.size()][2];
            for (int i = 0; i < mergedArray.size(); i++) {
                result[i] = mergedArray.get(i);
            }
            return result;
        }
    }
}
