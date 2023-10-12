package leetcode_grind;

public class Day334 {
    static class Solution {
        // https://leetcode.com/problems/peak-index-in-a-mountain-array/description/
        public int peakIndexInMountainArray(int[] arr) {
            var lo = 0;
            var hi = arr.length - 1;
            while (lo < hi) {
                var mid = (lo + hi) / 2;
                if (arr[mid] <= arr[mid + 1]) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }
    }
}
