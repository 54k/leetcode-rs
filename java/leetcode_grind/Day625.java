package leetcode_grind;

import java.util.Arrays;

public class Day625 {
    // https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/description/?envType=daily-question&envId=2024-08-03
    static class Solution1 {
        public boolean canBeEqual(int[] target, int[] arr) {
            Arrays.sort(target);
            Arrays.sort(arr);
            for (int i = 0; i < arr.length; i++) {
                if (target[i] != arr[i]) {
                    return false;
                }
            }
            return true;
        }
    }
}
