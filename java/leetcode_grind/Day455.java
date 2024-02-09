package leetcode_grind;

import java.util.Arrays;
import java.util.BitSet;
import java.util.List;

public class Day455 {
    // https://leetcode.com/problems/largest-divisible-subset/description/
    class Solution {
        public List<Integer> largestDivisibleSubset(int[] nums) {
            // dp i -- len of largest subset, 0=0
            Arrays.sort(nums);
            var n = nums.length;
            int[] dp = new int[n];
            int best = 0;
            BitSet[] maskDp = new BitSet[n];
            BitSet bestMask = new BitSet();
            for (int i = 0; i < n; i++) {
                BitSet bs = new BitSet();
                bs.set(i);
                maskDp[i] = (BitSet) bs.clone();
                dp[i] = 1;
                for (int j = 0; j < i; j++) {
                    bs = new BitSet();
                    bs.set(i);
                    bs.set(j);
                    if (nums[i] % nums[j] == 0 || nums[j] % nums[i] == 0) {
                        if (dp[i] <= dp[j] + 1) {
                            dp[i] = dp[j] + 1;
                            bs.or(maskDp[j]);
                            maskDp[i] = (BitSet) bs.clone();
                        }
                    }
                }
                if (best < dp[i]) {
                    best = dp[i];
                    bestMask = maskDp[i];
                }
            }
            return bestMask.stream().boxed().map(x -> nums[x]).toList();
        }
    }
}
