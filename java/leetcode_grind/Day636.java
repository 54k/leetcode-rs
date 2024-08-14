package leetcode_grind;

public class Day636 {
    // https://leetcode.com/problems/find-k-th-smallest-pair-distance/description/?envType=daily-question&envId=2024-08-14
    static class Solution1 {
        public int smallestDistancePair(int[] nums, int k) {
            int arraylength = nums.length;
            int maxElement = Integer.MIN_VALUE;
            for (int num : nums) {
                maxElement = Math.max(maxElement, num);
            }
            int[] distanceBucket = new int[maxElement + 1];
            for (int i = 0; i < arraylength; i++) {
                for (int j = i + 1; j < arraylength; j++) {
                    int distance = Math.abs(nums[i] - nums[j]);
                    ++distanceBucket[distance];
                }
            }
            for (int dist = 0; dist <= maxElement; dist++) {
                k -= distanceBucket[dist];
                if (k <= 0) {
                    return dist;
                }
            }
            return -1;
        }
    }

}
