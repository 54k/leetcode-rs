package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;

public class Day626 {
    // https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/description/?envType=daily-question&envId=2024-08-04
    static class Solution1 {
        public int rangeSum(int[] nums, int n, int left, int right) {
            ArrayList<Integer> storeSubarray = new ArrayList<>();
            for (int i = 0; i < nums.length; i++) {
                int sum = 0;
                for (int j = i; j < nums.length; j++) {
                    sum += nums[j];
                    storeSubarray.add(sum);
                }
            }
            Collections.sort(storeSubarray);

            int rangeSum = 0, mod = (int) 1e9 + 7;
            for (int i = left - 1; i <= right - 1; i++) {
                rangeSum = (rangeSum + storeSubarray.get(i)) % mod;
            }
            return rangeSum;
        }
    }
}
